use axum::async_trait;

use crate::model::{channel_parent::ChannelParent, error};


#[async_trait]
pub trait Repository: Send + Sync 
{
    async fn create_chat(&self, chat: ChannelParent) -> Result<ChannelParent, error::Server>;
    async fn update_chat(&self, chat: ChannelParent) -> Result<(), error::Server>;
    async fn get_chat_by_id(&self, chat_id: &str) -> Result<ChannelParent, error::Server>;
    async fn does_chat_exist(&self, chat: &ChannelParent) -> Result<bool, error::Server>;
}