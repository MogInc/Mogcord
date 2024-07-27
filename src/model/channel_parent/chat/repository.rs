use axum::async_trait;

use crate::model::error;
use super::Chat;


#[async_trait]
pub trait Repository: Send + Sync 
{
    async fn create_chat<'input, 'err>(&'input self, chat: Chat) -> Result<Chat, error::Server<'err>>;
    async fn update_chat<'input, 'err>(&'input self, chat: Chat) -> Result<(), error::Server<'err>>;
    async fn get_chat_by_id<'input, 'err>(&'input self, chat_id: &'input str) -> Result<Chat, error::Server<'err>>;
    async fn does_chat_exist<'input, 'err>(&'input self, chat: &'input Chat) -> Result<bool, error::Server<'err>>;
}