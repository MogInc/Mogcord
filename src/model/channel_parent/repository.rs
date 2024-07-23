use axum::async_trait;

use crate::model::error;
use super::ChannelParent;

#[async_trait]
pub trait Repository: Send + Sync 
{
    //chat
    async fn create_chat(&self, chat: ChannelParent) -> Result<ChannelParent, error::Server>;
    async fn update_chat(&self, chat: ChannelParent) -> Result<(), error::Server>;
    async fn get_chat_by_id(&self, chat_id: &str) -> Result<ChannelParent, error::Server>;
    async fn does_chat_exist(&self, chat: &ChannelParent) -> Result<bool, error::Server>;

    //server
    async fn create_server(&self, server: ChannelParent) -> Result<ChannelParent, error::Server>;
    async fn add_user_to_server(&self, server_id: &str, user_id: &str) -> Result<(), error::Server>;
    async fn get_server_by_id(&self, server_id: &str) -> Result<ChannelParent, error::Server>;
    async fn get_server_by_chat_info_id(&self, chat_info_id: &str) -> Result<ChannelParent, error::Server>;
}