use axum::async_trait;
use mongodb::bson::doc;
use crate::db::mongoldb::mongol_helper::MongolHelper;
use crate::{db::mongoldb::{MongolBucket, MongolDB, MongolMessage}, model::{chat::Bucket, message::{Message, MessageRepository}, misc::{Pagination, ServerError}}};

#[async_trait]
impl MessageRepository for MongolDB
{
    async fn create_message(&self, message: Message) -> Result<Message, ServerError>
    {
        let db_message = MongolMessage::try_from(message.clone())
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

        let mut session = self
            .client()
            .start_session()
            .await
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

        session
            .start_transaction()
            .await
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

        let date = message
            .timestamp
            .convert_to_bson_datetime()
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;


        match db_message.bucket_id
        {
            Some(_) =>
            {
                let bucket_filter = doc!
                {
                    "chat_id": db_message.chat_id,
                    "date": date,
                };

                let bucket_update = doc! 
                {
                    "$push": { "message_ids": db_message._id }
                };

                self
                    .buckets()
                    .update_one(bucket_filter, bucket_update)
                    .session(&mut session)
                    .await
                    .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;
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
                    .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;
            },
        };


        match self.messages().insert_one(&db_message).session(&mut session).await
        {
            Ok(_) => 
            {
                session
                    .commit_transaction()
                    .await
                    .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

                return Ok(message);
            },
            Err(err) => 
            {
                session
                    .abort_transaction()
                    .await
                    .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

                return Err(ServerError::UnexpectedError(err.to_string()));
            },
        }
    }

    async fn get_messages(&self, chat_id: &String, pagination: Pagination) 
        -> Result<Vec<Message>, ServerError>
    {
        Err(ServerError::ChatAlreadyExists)
    }
}
