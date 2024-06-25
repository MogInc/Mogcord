use axum::async_trait;

use crate::{db::mongoldb::{MongolDB, MongolMessage}, model::{message::{Message, MessageRepository}, misc::{Pagination, ServerError}}};

#[async_trait]
impl MessageRepository for MongolDB
{
    async fn create_message(&self, message: Message) 
        -> Result<Message, ServerError>
    {
        let db_message = MongolMessage::try_from(message)
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;



        Err(ServerError::ChatAlreadyExists)
    }

    async fn get_messages(&self, chat_id: &String, pagination: Pagination) 
        -> Result<Vec<Message>, ServerError>
    {
        Err(ServerError::ChatAlreadyExists)
    }
}