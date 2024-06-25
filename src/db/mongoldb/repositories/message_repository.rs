use axum::async_trait;

use crate::{db::mongoldb::MongolDB, model::{message::{Message, MessageRepository}, misc::{Pagination, ServerError}}};

#[async_trait]
impl MessageRepository for MongolDB
{
    async fn create_message(&self, message: Message) 
        -> Result<Message, ServerError>
    {
        Err(ServerError::ChatAlreadyExists)
    }

    async fn get_messages(&self, chat_id: &String, pagination: Pagination) 
        -> Result<Vec<Message>, ServerError>
    {
        Err(ServerError::ChatAlreadyExists)
    }
}