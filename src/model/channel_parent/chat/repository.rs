use axum::async_trait;

use crate::model::error;
use super::Chat;


#[async_trait]
pub trait Repository: Send + Sync 
{
    async fn create_chat<'input, 'err>(&'input self, chat: Chat) -> error::Result<'err, Chat>;
    async fn update_chat<'input, 'err>(&'input self, chat: Chat) -> error::Result<'err, ()>;
    async fn get_chat_by_id<'input, 'err>(&'input self, chat_id: &'input str) -> error::Result<'err, Chat>;
    async fn does_chat_exist<'input, 'err>(&'input self, chat: &'input Chat) -> error::Result<'err, bool>;
}