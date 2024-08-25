use axum::async_trait;

use crate::model::error;

use super::{chat, server, ChannelParent};

#[async_trait]
pub trait Repository:
    Send + Sync + server::Repository + chat::Repository
{
    async fn get_channel_parent<'input, 'err>(
        &'input self,
        channel_id: &'input str,
    ) -> error::Result<'err, ChannelParent>;
}
