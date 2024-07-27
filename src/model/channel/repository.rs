use axum::async_trait;

use crate::model::error;

use super::Channel;

#[async_trait]
pub trait Repository: Send + Sync
{
    async fn get_channel<'input, 'stack>(&'input self, channel_id: &'input str) -> Result<Channel, error::Server<'stack>>;
}