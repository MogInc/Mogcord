use axum::async_trait;

use crate::model::error;

use super::{chat, server, ChannelParent};

#[async_trait]
pub trait Repository: Send + Sync + server::Repository + chat::Repository
{
    async fn get_channel_parent(&self, channel_id: &str) -> Result<ChannelParent, error::Server>;
}