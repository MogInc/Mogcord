use axum::async_trait;

use super::MongolLog;
use crate::db::mongol::MongolDB;
use crate::model::error;
use crate::model::log::{
    self,
    RequestLogLine,
};
use crate::{
    bubble,
    server_error,
};

#[async_trait]
impl log::Repository for MongolDB
{
    async fn create_log<'input, 'err>(
        &'input self,
        log: RequestLogLine<'input>,
    ) -> error::Result<'err, ()>
    {
        let mongol_log = bubble!(MongolLog::try_from(log))?;

        match self.logs().insert_one(mongol_log).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(server_error!(
                error::Kind::Insert,
                error::OnType::Log
            )
            .add_debug_info("error", err.to_string())),
        }
    }
}
