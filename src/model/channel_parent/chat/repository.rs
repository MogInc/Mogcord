use axum::async_trait;

use crate::model::error;
use super::Chat;


#[async_trait]
pub trait Repository: Send + Sync 
{
    async fn create_chat<'input, 'stack>(&'input self, chat: Chat) -> Result<Chat, error::Server<'stack>>;
    async fn update_chat<'input, 'stack>(&'input self, chat: Chat) -> Result<(), error::Server<'stack>>;
    async fn get_chat_by_id<'input, 'stack>(&'input self, chat_id: &'input str) -> Result<Chat, error::Server<'stack>>;
    async fn does_chat_exist<'input, 'stack>(&'input self, chat: &'input Chat) -> Result<bool, error::Server<'stack>>;
}