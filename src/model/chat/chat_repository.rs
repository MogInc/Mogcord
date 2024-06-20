use axum::async_trait;

use super::{Chat, ChatError};

#[async_trait]
pub trait ChatRepository: Send + Sync 
{
    async fn create_chat(&self, chat: Chat) -> Result<Chat, ChatError>;
    async fn get_chat_by_id(&self, chat_id: &String) -> Result<Chat, ChatError>;
}