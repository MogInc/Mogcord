use axum::async_trait;

use crate::model::error;

use super::Server;

#[async_trait]
pub trait Repository: Send + Sync 
{
    async fn create_server(&self, server: Server) -> Result<Server, error::Server>;
    async fn get_server_by_id(&self, server_id: &str) -> Result<Server, error::Server>;
    async fn get_server_by_chat_info_id(&self, chat_info_id: &str) -> Result<Server, error::Server>;
}