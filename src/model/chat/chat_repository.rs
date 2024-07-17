use axum::async_trait;

use crate::model::misc::ServerError;

use super::{Chat, ChatInfo};

#[async_trait]
pub trait ChatRepository: Send + Sync 
{
    async fn create_chat(&self, chat: Chat) -> Result<Chat, ServerError>;
    async fn get_chat_by_id(&self, chat_id: &str) -> Result<Chat, ServerError>;
    async fn get_chat_info_by_id(&self, chat_info_id: &str) -> Result<ChatInfo, ServerError>;
    async fn does_chat_exist(&self, chat: &Chat) -> Result<bool, ServerError>;
}