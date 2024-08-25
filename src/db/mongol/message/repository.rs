use axum::async_trait;
use bson::{
    Document,
    Regex,
};
use chrono::Utc;
use futures_util::StreamExt;
use mongodb::bson::{
    doc,
    from_document,
};

use crate::db::mongol::helper::{
    self,
    MongolHelper,
};
use crate::db::mongol::{
    MongolBucket,
    MongolDB,
    MongolMessage,
};
use crate::model::bucket::Bucket;
use crate::model::error::{
    self,
};
use crate::model::message::{
    self,
    Message,
};
use crate::model::Pagination;
use crate::{
    bubble,
    map_mongo_collection_keys_to_string,
    map_mongo_key_to_string,
    server_error,
    transaction_error,
};

#[async_trait]
impl message::Repository for MongolDB
{
    async fn create_message<'input, 'err>(
        &'input self,
        mut message: Message,
    ) -> error::Result<'err, Message>
    {
        let mut db_message = bubble!(MongolMessage::try_from(
            &message
        ))?;

        let mut session = self
            .client()
            .start_session()
            .await
            .map_err(|err| transaction_error!(err))?;

        session
            .start_transaction()
            .await
            .map_err(|err| transaction_error!(err))?;

        let date = message.timestamp.convert_to_bson_date().map_err(|err| {
            server_error!(
                error::Kind::Parse,
                error::OnType::Date
            )
            .add_debug_info("error", err.to_string())
        })?;

        let bucket_filter = doc! {
            "channel_id": db_message.channel_id,
            "date": date,
        };

        let bucket_option = self
            .buckets()
            .find_one(bucket_filter.clone())
            .await
            .map_err(|err| {
                server_error!(
                    error::Kind::Fetch,
                    error::OnType::Bucket
                )
                .add_debug_info("error", err.to_string())
            })?;

        let bucket_current = if let Some(bucket) = bucket_option
        {
            let bucket_update = doc! {
                "$push": { "message_ids": db_message._id }
            };

            self.buckets()
                .update_one(bucket_filter, bucket_update)
                .session(&mut session)
                .await
                .map_err(|err| {
                    server_error!(
                        error::Kind::Update,
                        error::OnType::Bucket
                    )
                    .add_debug_info("error", err.to_string())
                })?;

            bucket
        }
        else
        {
            let mut bucket = Bucket::new(
                &message.channel,
                &message.timestamp,
            );

            bucket.add_message(message.clone());

            let db_bucket = MongolBucket::try_from(&bucket)
                .map_err(|err| server_error!(err))?;

            self.buckets()
                .insert_one(&db_bucket)
                .session(&mut session)
                .await
                .map_err(|err| {
                    server_error!(
                        error::Kind::Insert,
                        error::OnType::Bucket
                    )
                    .add_debug_info("error", err.to_string())
                })?;

            db_bucket
        };

        db_message.bucket_id = Some(bucket_current._id);

        //can remove this match and have implicit abort
        match self
            .messages()
            .insert_one(&db_message)
            .session(&mut session)
            .await
        {
            Ok(_) =>
            {
                session
                    .commit_transaction()
                    .await
                    .map_err(|err| transaction_error!(err))?;

                message.bucket_id = Some(bucket_current._id.to_string());

                Ok(message)
            },
            Err(err) =>
            {
                session
                    .abort_transaction()
                    .await
                    .map_err(|err| transaction_error!(err))?;

                Err(server_error!(
                    error::Kind::Insert,
                    error::OnType::Message
                )
                .add_debug_info("error", err.to_string()))
            },
        }
    }

    async fn get_valid_messages<'input, 'err>(
        &'input self,
        channel_id: &'input str,
        pagination: Pagination,
    ) -> error::Result<'err, Vec<Message>>
    {
        let channel_id_local =
            bubble!(helper::convert_domain_id_to_mongol(channel_id))?;

        let mut pipelines = vec![
            doc! {
                "$match":
                {
                    "channel_id": channel_id_local,
                    "flag": internal_valid_message_filter(),
                },
            },
            //sort on date from new to old
            //sorting is in general expensive, no clue how expensive this is gonna get
            //i have added an clustered index on timestamp (chan_id, timestamp, flag) thats DESC
            doc! {
                "$sort":
                {
                    "timestamp": -1
                }
            },
            //early skip + limit since i assume it's cheaper
            doc! {
                "$skip":  i32::try_from(pagination.get_skip_size()).ok().unwrap_or(0)
            },
            doc! {
                "$limit": i32::try_from(pagination.page_size).ok().unwrap_or(0)
            },
        ];

        pipelines.extend(internal_message_pipeline());

        let mut cursor =
            self.messages().aggregate(pipelines).await.map_err(|err| {
                server_error!(
                    error::Kind::Fetch,
                    error::OnType::Message
                )
                .add_debug_info("error", err.to_string())
            })?;

        //what would be faster
        //1: reallocating vecs when capacity is reached
        //2: having a count or length and setting a capacity

        let mut messages: Vec<Message> = Vec::new();

        while let Some(result) = cursor.next().await
        {
            match result
            {
                Ok(document) =>
                {
                    let message: Message =
                        from_document(document).map_err(|err| {
                            server_error!(
                                error::Kind::Parse,
                                error::OnType::Message
                            )
                            .add_debug_info("error", err.to_string())
                        })?;
                    messages.push(message);
                },
                Err(err) => println!("{err}"),
            };
        }

        Ok(messages)
    }

    async fn update_message<'input, 'err>(
        &'input self,
        message: Message,
    ) -> error::Result<'err, Message>
    {
        let db_message = bubble!(MongolMessage::try_from(
            &message
        ))?;

        let filter = doc! {
            "_id": db_message._id,
        };

        let update = doc! {
            "$set":
            {
                "value": db_message.value,
                "flag": db_message.flag,
            }
        };

        match self.messages().update_one(filter, update).await
        {
            Ok(_) => Ok(message),
            Err(err) => Err(server_error!(
                error::Kind::Update,
                error::OnType::Message
            )
            .add_debug_info("error", err.to_string())),
        }
    }

    async fn get_message<'input, 'err>(
        &'input self,
        message_id: &'input str,
    ) -> error::Result<'err, Message>
    {
        let message_id_local =
            bubble!(helper::convert_domain_id_to_mongol(message_id))?;

        let mut pipelines = vec![doc! {
            "$match":
            {
                "_id": message_id_local
            },
        }];

        pipelines.extend(internal_message_pipeline());

        let mut cursor =
            self.messages().aggregate(pipelines).await.map_err(|err| {
                server_error!(
                    error::Kind::Fetch,
                    error::OnType::Message
                )
                .add_debug_info("error", err.to_string())
            })?;

        let document_option =
            cursor.next().await.transpose().map_err(|err| {
                server_error!(
                    error::Kind::Unexpected,
                    error::OnType::Message
                )
                .add_debug_info("error", err.to_string())
            })?;

        match document_option
        {
            Some(document) =>
            {
                let message: Message =
                    from_document(document).map_err(|err| {
                        server_error!(
                            error::Kind::Parse,
                            error::OnType::Message
                        )
                        .add_debug_info("error", err.to_string())
                    })?;

                Ok(message)
            },
            None => Err(server_error!(
                error::Kind::NotFound,
                error::OnType::Message
            )
            .add_debug_info(
                "message id",
                message_id.to_string(),
            )),
        }
    }
}

fn internal_valid_message_filter() -> Document
{
    let valid_flags = [
        message::Flag::None,
        message::Flag::Edited {
            date: Utc::now(),
        },
    ];

    let valid_flags_bson: Vec<Regex> = valid_flags
        .iter()
        .map(|flag| {
            let temp = flag.to_string();

            let parts: Vec<&str> = temp.split('|').collect();

            let pattern = format!("^{}", parts[0]);
            Regex {
                pattern,
                options: String::new(),
            }
        })
        .collect();

    doc! { "$in": valid_flags_bson }
}

fn internal_message_pipeline() -> [Document; 8]
{
    [
        //join with chat
        doc! {
            "$lookup":
            {
                "from": "chats",
                "localField": "channel_id",
                "foreignField": "_id",
                "as": "channel"
            }
        },
        //join with owner of message
        doc! {
            "$lookup":
            {
                "from": "users",
                "localField": "owner_id",
                "foreignField": "_id",
                "as": "owner"
            }
        },
        //should only have 1 chat
        doc! {
            "$unwind":
            {
                "path": "$chat"
            }
        },
        //should only have 1 owner
        doc! {
            "$unwind":
            {
                "path": "$owner"
            }
        },
        //join with owners of chat
        doc! {
            "$lookup":
            {
                "from": "users",
                "localField": "chat.owner_ids",
                "foreignField": "_id",
                "as": "chat.owners"
            }
        },
        //join with users of chat
        doc! {
            "$lookup":
            {
                "from": "users",
                "localField": "chat.user_ids",
                "foreignField": "_id",
                "as": "chat.users"
            }
        },
        //converts from special ids to string
        doc! {
            "$addFields":
            {
                "id": map_mongo_key_to_string!("$_id", "uuid"),
                "bucket_id": map_mongo_key_to_string!("$bucket_id", "uuid"),
                "chat.id": map_mongo_key_to_string!("$chat._id", "uuid"),
                "owner.id": map_mongo_key_to_string!("$owner._id", "uuid"),
                "chat.owners": map_mongo_collection_keys_to_string!("$chat.owners", "_id", "id", "uuid"),
                "chat.users": map_mongo_collection_keys_to_string!("$chat.users", "_id", "id", "uuid"),
            }
        },
        //hide unneeded fields
        doc! {
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
    ]
}
