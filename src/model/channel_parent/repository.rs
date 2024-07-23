use axum::async_trait;

use crate::model::error;
use super::{server, ChannelParent};

#[async_trait]
pub trait Repository: Send + Sync + server::Repository
{
    async fn create_chat(&self, chat: ChannelParent) -> Result<ChannelParent, error::Server>;
    async fn update_chat(&self, chat: ChannelParent) -> Result<(), error::Server>;
    async fn get_chat_by_id(&self, chat_id: &str) -> Result<ChannelParent, error::Server>;
    async fn does_chat_exist(&self, chat: &ChannelParent) -> Result<bool, error::Server>;
}