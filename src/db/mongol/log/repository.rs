use axum::async_trait;

use crate::model::{error::{self}, log::{self, RequestLogLine}};
use crate::db::mongol::MongolDB;

#[async_trait]
impl log::Repository for MongolDB
{
    async fn create_log<'input, 'err>(&'input self, log: RequestLogLine) -> Result<(), error::Server<'err>>
    {
        match self.logs().insert_one(log).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(error::Server::new(
                error::Kind::Insert,
                error::OnType::Log, 
                file!(),
                line!())
                .add_debug_info(err.to_string())
            ),
        }
    }
}