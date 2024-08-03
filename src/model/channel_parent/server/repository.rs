use axum::async_trait;

use crate::model::error;

use super::Server;

#[async_trait]
pub trait Repository: Send + Sync 
{
    async fn create_server<'input, 'err>(&'input self, server: Server) -> error::Result<'err, Server>;
    async fn add_user_to_server<'input, 'err>(&'input self, server_id: &'input str, user_id: &'input str) -> error::Result<'err, ()>;
    async fn get_server_by_id<'input, 'err>(&'input self, server_id: &'input str) -> error::Result<'err, Server>;
    async fn get_server_by_channel_id<'input, 'err>(&'input self, channel_id: &'input str) -> error::Result<'err, Server>;
}