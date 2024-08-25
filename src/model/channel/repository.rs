use axum::async_trait;

use crate::model::error;

use super::Channel;

#[async_trait]
pub trait Repository: Send + Sync
{
    async fn get_channel<'input, 'err>(
        &'input self,
        channel_id: &'input str,
    ) -> error::Result<'err, Channel>;
}
