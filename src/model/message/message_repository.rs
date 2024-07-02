use axum::async_trait;

use crate::model::misc::{Pagination, ServerError};

use super::Message;

#[async_trait]
pub trait MessageRepository: Send + Sync
{
    async fn create_message(&self, message: Message) -> Result<Message, ServerError>;
    async fn update_message(&self, message: Message) -> Result<Message, ServerError>;
    async fn get_message(&self, message_id: &str) -> Result<Message, ServerError>;
    async fn get_messages(&self, chat_id: &str, pagination: Pagination) 
        -> Result<Vec<Message>, ServerError>;
}