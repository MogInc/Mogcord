use axum::async_trait;

use crate::model::{error, pagination::Pagination};

use super::Message;

#[async_trait]
pub trait Repository: Send + Sync
{
    async fn create_message<'input, 'err>(&'input self, message: Message) -> error::Result<'err, Message>;
    async fn update_message<'input, 'err>(&'input self, message: Message) -> error::Result<'err, Message>;
    async fn get_message<'input, 'err>(&'input self, message_id: &'input str) -> error::Result<'err, Message>;
    async fn get_valid_messages<'input, 'err>(&'input self, channel_id: &'input str, pagination: Pagination) 
        -> error::Result<'err, Vec<Message>>;
}