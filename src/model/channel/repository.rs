use axum::async_trait;

use crate::model::error;

use super::{Channel, Parent};

#[async_trait]
pub trait Repository: Send + Sync
{
    async fn get_channel(&self, channel_id: &str) -> Result<Channel, error::Server>;
    async fn get_channel_parent(&self, channel_id: &str) -> Result<Box<dyn Parent>, error::Server>;
}