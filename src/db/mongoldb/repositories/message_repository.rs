use axum::async_trait;
use futures_util::StreamExt;
use mongodb::bson::{doc, from_document};
use crate::{convert_mongo_key_to_string, map_mongo_collection_keys};
use crate::db::mongoldb::mongol_helper::{self, MongolHelper};
use crate::{db::mongoldb::{MongolBucket, MongolDB, MongolMessage}, model::{chat::Bucket, message::{Message, MessageRepository}, misc::{Pagination, ServerError}}};

#[async_trait]
impl MessageRepository for MongolDB
{
    async fn create_message(&self, mut message: Message) -> Result<Message, ServerError>
    {
        let mut db_message = MongolMessage::try_from(&message)
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

        let mut session = self
            .client()
            .start_session()
            .await
            .map_err(|err| ServerError::TransactionError(err.to_string()))?;

        session
            .start_transaction()
            .await
            .map_err(|err| ServerError::TransactionError(err.to_string()))?;

        let date = message
            .timestamp
            .convert_to_bson_date()
            .map_err(|err| ServerError::TransactionError(err.to_string()))?;

        let bucket_filter = doc!
        {
            "chat_id": db_message.chat_id,
            "date": date,
        };

        let bucket_option = self
            .buckets()
            .find_one(bucket_filter.clone())
            .await
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;


        let bucket_current : MongolBucket;

        match bucket_option
        {
            Some(bucket) =>
            {
                let bucket_update = doc! 
                {
                    "$push": { "message_ids": db_message._id }
                };

                self
                    .buckets()
                    .update_one(bucket_filter, bucket_update)
                    .session(&mut session)
                    .await
                    .map_err(|err| ServerError::FailedUpdate(err.to_string()))?;

                bucket_current = bucket;
            },
            None =>
            {
                let mut bucket = Bucket::new(&message.chat, &message.timestamp);
                
                bucket.add_message(message.clone());

                let db_bucket = MongolBucket::try_from(&bucket)
                    .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

                self
                    .buckets()
                    .insert_one(&db_bucket)
                    .session(&mut session)
                    .await
                    .map_err(|err| ServerError::FailedInsert(err.to_string()))?;

                bucket_current = db_bucket;
            },
        };

        db_message.bucket_id = Some(bucket_current._id);

        match self.messages().insert_one(&db_message).session(&mut session).await
        {
            Ok(_) => 
            {
                session
                    .commit_transaction()
                    .await
                    .map_err(|err| ServerError::TransactionError(err.to_string()))?;

                message.bucket_id = Some(bucket_current._id.to_string());

                return Ok(message);
            },
            Err(err) => 
            {
                session
                    .abort_transaction()
                    .await
                    .map_err(|err| ServerError::TransactionError(err.to_string()))?;

                return Err(ServerError::UnexpectedError(err.to_string()));
            },
        }
    }

    async fn get_messages(&self, chat_id: &str, pagination: Pagination) 
        -> Result<Vec<Message>, ServerError>
    {

        let chat_id_local = mongol_helper::convert_domain_id_to_mongol(&chat_id)
            .map_err(|_| ServerError::ChatNotFound)?;
        
        let pipelines = vec![
            //filter to only given chat
            doc! 
            {
                "$match":
                {
                    "chat_id": chat_id_local
                },
            },
            //sort on date from new to old
            //sorting is in general expensive, no clue how expensive this is gonna get
            doc!
            {
                "$sort":
                {
                    "timestamp": -1
                }
            },
            //early skip + limit since i assume it's cheaper
            doc!
            {
                "$skip": pagination.get_skip_size() as i32
            },
            doc! 
            {
                "$limit": pagination.page_size as i32
            },
            //join with chat
            doc! 
            {
                "$lookup": 
                {
                    "from": "chats",
                    "localField": "chat_id",
                    "foreignField": "_id",
                    "as": "chat"
                }
            },
            //join with owner of message
            doc! 
            {
                "$lookup": 
                {
                    "from": "users",
                    "localField": "owner_id",
                    "foreignField": "_id",
                    "as": "owner"
                }
            },
            //should only have 1 chat
            doc!
            {
                "$unwind": 
                {
                    "path": "$chat"
                }
            },
            //should only have 1 owner
            doc!
            {
                "$unwind": 
                {
                    "path": "$owner"
                }
            },
            //join with owners of chat
            doc! 
            {
                "$lookup": 
                {
                    "from": "users",
                    "localField": "chat.owner_ids",
                    "foreignField": "_id",
                    "as": "chat.owners"
                }
            },
            //join with users of chat
            doc! 
            {
                "$lookup": 
                {
                    "from": "users",
                    "localField": "chat.user_ids",
                    "foreignField": "_id",
                    "as": "chat.users"
                }
            },
            //converts from special ids to string
            doc!
            {
                "$addFields":
                {
                    "id": convert_mongo_key_to_string!("$_id", "uuid"),
                    "bucket_id": convert_mongo_key_to_string!("$bucket_id", "uuid"),
                    "chat.id": convert_mongo_key_to_string!("$chat._id", "uuid"),
                    "owner.id": convert_mongo_key_to_string!("$owner._id", "uuid"),
                    "chat.owners": map_mongo_collection_keys!("$chat.owners", "id", "uuid"),
                    "chat.users": map_mongo_collection_keys!("$chat.users", "id", "uuid"),
                }
            },
            //hide unneeded fields
            doc! 
            {
                "$unset": 
                [
                    "_id",
                    "owner_id",
                    "chat_id",
                    "chat._id",
                    "chat.owner_ids",
                    "chat.user_ids",
                    "chat.bucket_ids",
                    "chat.owners._id",
                    "chat.users._id",
                    "owner._id"
                ]
            },
        ];


        let mut cursor = self
            .messages()
            .aggregate(pipelines)
            .await
            .map_err(|err| ServerError::FailedRead(err.to_string()))?;

        let mut messages: Vec<Message> = Vec::new();

        while let Some(result) = cursor.next().await
        {
            match result
            {
                Ok(document) => 
                {
                    let message: Message = from_document(document)
                        .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

                    messages.push(message);
                },
                Err(err) => println!("{}", err),
            };
        }


        return Ok(messages);
    }

    async fn update_message(&self, message: Message) -> Result<Message, ServerError>
    {
        let db_message = MongolMessage::try_from(&message)
        .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;
    
        let filter = doc! 
        {
            "_id": db_message._id,
        };

        let update = doc! 
        {
            "$set":
            {
                "value": db_message.value,
                "flag": db_message.flag,
            }
        };

        match self.messages().update_one(filter, update).await
        {
            Ok(_) => Ok(message),
            Err(err) => Err(ServerError::FailedInsert(err.to_string())),
        }
    }

    async fn get_message(&self, message_id: &str) -> Result<Message, ServerError>
    {

        let message_id_local = mongol_helper::convert_domain_id_to_mongol(&message_id)
            .map_err(|_| ServerError::MessageNotFound)?;
        
        let pipelines = vec![
            //filter to only given chat
            doc! 
            {
                "$match":
                {
                    "_id": message_id_local
                },
            },
            //join with chat
            doc! 
            {
                "$lookup": 
                {
                    "from": "chats",
                    "localField": "chat_id",
                    "foreignField": "_id",
                    "as": "chat"
                }
            },
            //join with owner of message
            doc! 
            {
                "$lookup": 
                {
                    "from": "users",
                    "localField": "owner_id",
                    "foreignField": "_id",
                    "as": "owner"
                }
            },
            //should only have 1 chat
            doc!
            {
                "$unwind": 
                {
                    "path": "$chat"
                }
            },
            //should only have 1 owner
            doc!
            {
                "$unwind": 
                {
                    "path": "$owner"
                }
            },
            //join with owners of chat
            doc! 
            {
                "$lookup": 
                {
                    "from": "users",
                    "localField": "chat.owner_ids",
                    "foreignField": "_id",
                    "as": "chat.owners"
                }
            },
            //join with users of chat
            doc! 
            {
                "$lookup": 
                {
                    "from": "users",
                    "localField": "chat.user_ids",
                    "foreignField": "_id",
                    "as": "chat.users"
                }
            },
            //converts from special ids to string
            doc!
            {
                "$addFields":
                {
                    "id": convert_mongo_key_to_string!("$_id", "uuid"),
                    "bucket_id": convert_mongo_key_to_string!("$bucket_id", "uuid"),
                    "chat.id": convert_mongo_key_to_string!("$chat._id", "uuid"),
                    "owner.id": convert_mongo_key_to_string!("$owner._id", "uuid"),
                    "chat.owners": map_mongo_collection_keys!("$chat.owners", "id", "uuid"),
                    "chat.users": map_mongo_collection_keys!("$chat.users", "id", "uuid"),
                }
            },
            //hide unneeded fields
            doc! 
            {
                "$unset": 
                [
                    "_id",
                    "owner_id",
                    "chat_id",
                    "chat._id",
                    "chat.owner_ids",
                    "chat.user_ids",
                    "chat.bucket_ids",
                    "chat.owners._id",
                    "chat.users._id",
                    "owner._id"
                ]
            },
        ];


        let mut cursor = self
            .messages()
            .aggregate(pipelines)
            .await
            .map_err(|err| ServerError::FailedRead(err.to_string()))?;


        let document_option = cursor
            .next()
            .await
            .transpose()
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

        match document_option
        {
            Some(document) => 
            {
                let message: Message = from_document(document)
                    .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

                return Ok(message);
            },
            None => Err(ServerError::MessageNotFound),
        }
    }
}
