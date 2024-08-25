use axum::async_trait;

use crate::model::error;

use super::RequestLogLine;

#[async_trait]
pub trait Repository: Send + Sync
{
    async fn create_log<'input, 'err>(
        &'input self,
        log: RequestLogLine<'input>,
    ) -> error::Result<'err, ()>;
}
