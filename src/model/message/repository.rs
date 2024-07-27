use axum::async_trait;

use crate::model::{error, pagination::Pagination};

use super::Message;

#[async_trait]
pub trait Repository: Send + Sync
{
    async fn create_message<'input, 'stack>(&'input self, message: Message) -> Result<Message, error::Server<'stack>>;
    async fn update_message<'input, 'stack>(&'input self, message: Message) -> Result<Message, error::Server<'stack>>;
    async fn get_message<'input, 'stack>(&'input self, message_id: &'input str) -> Result<Message, error::Server<'stack>>;
    async fn get_valid_messages<'input, 'stack>(&'input self, channel_id: &'input str, pagination: Pagination) 
        -> Result<Vec<Message>, error::Server<'stack>>;
}