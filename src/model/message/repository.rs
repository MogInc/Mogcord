use axum::async_trait;

use crate::model::{error, pagination::Pagination};

use super::Message;

#[async_trait]
pub trait Repository: Send + Sync
{
    async fn create_message(&self, message: Message) -> Result<Message, error::Server>;
    async fn update_message(&self, message: Message) -> Result<Message, error::Server>;
    async fn get_message(&self, message_id: &str) -> Result<Message, error::Server>;
    async fn get_valid_messages(&self, chat_id: &str, pagination: Pagination) 
        -> Result<Vec<Message>, error::Server>;
}