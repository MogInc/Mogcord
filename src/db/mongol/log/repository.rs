use axum::async_trait;

use crate::model::{error::{self}, log::{self, RequestLogLine}};
use crate::db::mongol::MongolDB;

use super::MongolLog;

#[async_trait]
impl log::Repository for MongolDB
{
    async fn create_log<'input, 'err>(&'input self, log: RequestLogLine<'input>) -> Result<(), error::Server<'err>>
    {
        let mongol_log = MongolLog::try_from(log)?;

        match self.logs().insert_one(mongol_log).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(error::Server::new(
                error::Kind::Insert,
                error::OnType::Log, 
                file!(),
                line!())
                .add_debug_info("error", err.to_string())
            ),
        }
    }
}