use axum::async_trait;

use crate::db::MongolServer;
use crate::model::error;
use crate::model::server::{self, Server};
use crate::db::mongol::MongolDB;

#[async_trait]
impl server::Repository for MongolDB
{
    async fn create_server(&self, server: Server) -> Result<Server, error::Server>
    {
        let db_server = MongolServer::try_from(&server)?;

        match self.servers().insert_one(&db_server).await
        {
            Ok(_) => Ok(server),
            Err(err) => Err(error::Server::FailedInsert(err.to_string())),
        }
    }
    async fn get_server_by_id(&self, server_id: &str) -> Result<Server, error::Server>
    {
        todo!()
    }
    async fn get_server_by_chat_info_id(&self, chat_info_id: &str) -> Result<Server, error::Server>
    {
        todo!()
    }
}
