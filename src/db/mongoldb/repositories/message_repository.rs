use axum::async_trait;
use futures_util::StreamExt;
use mongodb::bson::{doc, from_document, Uuid};
use crate::{convert_mongo_key_to_string, map_mongo_collection};
use crate::db::mongoldb::mongol_helper::MongolHelper;
use crate::{db::mongoldb::{MongolBucket, MongolDB, MongolMessage}, model::{chat::Bucket, message::{Message, MessageRepository}, misc::{Pagination, ServerError}}};

#[async_trait]
impl MessageRepository for MongolDB
{
    async fn create_message(&self, mut message: Message) -> Result<Message, ServerError>
    {
        let mut db_message = MongolMessage::try_from(message.clone())
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
            .convert_to_bson_datetime()
            .map_err(|err| ServerError::TransactionError(err.to_string()))?;

        let bucket_filter = doc!
        {
            "chat_id": db_message.chat_id,
            "date": date,
        };

        let bucket_option: Option<MongolBucket> = self
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

                let db_bucket = MongolBucket::try_from(bucket)
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

    async fn get_messages(&self, chat_id: &String, pagination: Pagination) 
        -> Result<Vec<Message>, ServerError>
    {

        let chat_id_local = Uuid::parse_str(chat_id)
            .map_err(|_| ServerError::ChatNotFound)?;
        
        let pipelines = vec![
            //filter to only given chats
            doc! 
            {
                "$match":
                {
                    "chat_id": chat_id_local
                },
            },
            //sort on date from new to old
            doc!
            {
                "$sort":
                {
                    "date": -1
                }
            },
            //join with messages
            doc! 
            {
                "$lookup":
                {
                    "from": "messages",
                    "localField": "message_ids",
                    "foreignField": "_id",
                    "as": "messages"
                },
            },
            //join chat
            doc! 
            {
                "$lookup":
                {
                    "from": "chats",
                    "localField": "chat_id",
                    "foreignField": "_id",
                    "as": "chat"
                },
            },
            //rename fields
            doc!
            {
                "$addFields":
                {
                    "messages": map_mongo_collection!("$messages", "id", "uuid"),
                },
            },

            //if the skip+limit is sooner you're not certain you retrieve the correct amount of messages
            //unless you wasnt to recursive pull but no clue how or if its even possible with aggregates
            //skip offset
            doc! 
            {
                "$skip": pagination.get_skip_size() as i32
            },
            //limit output
            doc! 
            {
                "$limit": pagination.page_size as i32
            },
        ];

        let mut cursor = self
            .buckets()
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
                    let message = from_document(document)
                        .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

                    println!("{:?}", message);
                    //messages.push(message);
                },
                Err(_) => (),
            };
        }


        return Ok(messages);
    }
}
