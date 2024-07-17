use axum::async_trait;

use crate::model::misc::ServerError;

use super::ChatInfo;

#[async_trait]
pub trait ChatRepository: Send + Sync 
{
    async fn create_chat(&self, chat: ChatInfo) -> Result<ChatInfo, ServerError>;
    async fn get_chat_by_id(&self, chat_id: &str) -> Result<ChatInfo, ServerError>;
    async fn does_chat_exist(&self, chat: &ChatInfo) -> Result<bool, ServerError>;
}