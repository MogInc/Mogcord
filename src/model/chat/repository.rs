use axum::async_trait;

use crate::model::error;
use super::Chat;

#[async_trait]
pub trait Repository: Send + Sync 
{
    async fn create_chat(&self, chat: Chat) -> Result<Chat, error::Server>;
    async fn update_chat(&self, chat: Chat) -> Result<(), error::Server>;
    async fn get_chat_by_id(&self, chat_id: &str) -> Result<Chat, error::Server>;
    async fn get_chat_by_chat_info_id(&self, chat_info_id: &str) -> Result<Chat, error::Server>;
    async fn does_chat_exist(&self, chat: &Chat) -> Result<bool, error::Server>;
}