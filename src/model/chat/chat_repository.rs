use axum::async_trait;

use crate::model::error::ServerError;
use super::Chat;

#[async_trait]
pub trait ChatRepository: Send + Sync 
{
    async fn create_chat(&self, chat: Chat) -> Result<Chat, ServerError>;
    async fn get_chat_by_id(&self, chat_id: &str) -> Result<Chat, ServerError>;
    async fn get_chat_by_chat_info_id(&self, chat_info_id: &str) -> Result<Chat, ServerError>;
    async fn does_chat_exist(&self, chat: &Chat) -> Result<bool, ServerError>;
}