use axum::async_trait;
use chrono::{Datelike, Utc};
use mongodb::bson::{self, doc, Bson};

use crate::{db::mongoldb::{MongolDB, MongolMessage}, model::{message::{Message, MessageRepository}, misc::{Pagination, ServerError}}};

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

        let message_date = message.timestamp.date_naive();

        let date = bson::DateTime::builder()
            .year(message_date.year())
            .month(message_date.month().try_into().unwrap())
            .day(message_date.day().try_into().unwrap())
            .build()
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

        let bucket_filter = doc!
        {
            "chat_id": db_message.chat_id,
            "date": date,
        };

        let bucket = self
            .buckets()
            .find_one(bucket_filter)
            .await
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

        
        if()
        {
            
        }


        Err(ServerError::ChatAlreadyExists)
    }

    async fn get_messages(&self, chat_id: &String, pagination: Pagination) 
        -> Result<Vec<Message>, ServerError>
    {
        Err(ServerError::ChatAlreadyExists)
    }
}
