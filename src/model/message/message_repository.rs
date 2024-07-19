use axum::async_trait;

use crate::model::{error::ServerError, pagination::Pagination};

use super::Message;

#[async_trait]
pub trait MessageRepository: Send + Sync
{
    async fn create_message(&self, message: Message) -> Result<Message, ServerError>;
    async fn update_message(&self, message: Message) -> Result<Message, ServerError>;
    async fn get_message(&self, message_id: &str) -> Result<Message, ServerError>;
    async fn get_valid_messages(&self, chat_id: &str, pagination: Pagination) 
        -> Result<Vec<Message>, ServerError>;
}