use axum::async_trait;
use mongodb::bson::{doc, Bson};

use crate::{db::mongoldb::{MongolDB, MongolMessage}, model::{message::{Message, MessageRepository}, misc::{Pagination, ServerError}}};

#[async_trait]
impl MessageRepository for MongolDB
{
    async fn create_message(&self, message: Message) -> Result<Message, ServerError>
    {
        let db_message = MongolMessage::try_from(message)
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

        let message_date = "";

        let bucket_filter = doc!
        {
            "chat_id": db_message.chat_id,
            "date": message_date,
        };

        let bucket = self
            .buckets()
            .find_one(bucket_filter)
            .await
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

        Err(ServerError::ChatAlreadyExists)
    }

    async fn get_messages(&self, chat_id: &String, pagination: Pagination) 
        -> Result<Vec<Message>, ServerError>
    {
        Err(ServerError::ChatAlreadyExists)
    }
}
