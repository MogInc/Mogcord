use axum::async_trait;

use crate::model::error;

use super::Server;

#[async_trait]
pub trait Repository: Send + Sync 
{
    async fn create_server(&self, server: Server) -> Result<Server, error::Server>;
    async fn add_user_to_server(&self, server_id: &str, user_id: &str) -> Result<(), error::Server>;
    async fn get_server_by_id(&self, server_id: &str) -> Result<Server, error::Server>;
    async fn get_server_by_channel_id(&self, channel_id: &str) -> Result<Server, error::Server>;
}