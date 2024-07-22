use axum::async_trait;

use crate::model::error;

use super::Channel;

#[async_trait]
pub trait Repository: Send + Sync
{
    async fn get_channel(&self, channel_id: &str) -> Result<Channel, error::Server>;
    // async fn get_channel_parent(&self, channel_id: &str) -> Result<ParentWrapper, error::Server>;
}