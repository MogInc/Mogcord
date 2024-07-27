use axum::async_trait;

use crate::model::error;

use super::Server;

#[async_trait]
pub trait Repository: Send + Sync 
{
    async fn create_server<'input, 'err>(&'input self, server: Server) -> Result<Server, error::Server<'err>>;
    async fn add_user_to_server<'input, 'err>(&'input self, server_id: &'input str, user_id: &'input str) -> Result<(), error::Server<'err>>;
    async fn get_server_by_id<'input, 'err>(&'input self, server_id: &'input str) -> Result<Server, error::Server<'err>>;
    async fn get_server_by_channel_id<'input, 'err>(&'input self, channel_id: &'input str) -> Result<Server, error::Server<'err>>;
}