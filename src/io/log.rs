use std::path::Path;

use crate::model::error;
use crate::model::log::{self, RequestLogLine};
use crate::server_error;
use axum::async_trait;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};

use super::FileWriter;

#[async_trait]
impl log::Repository for FileWriter
{
    async fn create_log<'input, 'err>(
        &'input self,
        log: RequestLogLine<'input>,
    ) -> error::Result<'err, ()>
    {
        let path = Path::new(&self.folder_path)
            .join(format!("{}.log", chrono::offset::Local::now().date_naive()));

        //most likely too naive of a solution
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&path)
            .await
            .map_err(|err| {
                server_error!(error::Kind::FileOpening, error::OnType::Log)
                    .add_debug_info("file error", err.to_string())
                    .add_debug_info("path", format!("{path:?}"))
            })?;

        let mut buf_writer = BufWriter::new(file);

        let json = serde_json::to_string(&log).map_err(|err| {
            server_error!(error::Kind::Parse, error::OnType::Log)
                .add_debug_info("file error", err.to_string())
        })?;

        buf_writer.write_all(json.as_bytes()).await.map_err(|err| {
            server_error!(error::Kind::Write, error::OnType::Log)
                .add_debug_info("file error", err.to_string())
        })?;

        buf_writer.write_all(b"\n").await.map_err(|err| {
            server_error!(error::Kind::Write, error::OnType::Log)
                .add_debug_info("file error", err.to_string())
        })?;

        buf_writer.flush().await.map_err(|err| {
            server_error!(error::Kind::FlushBuffer, error::OnType::Log)
                .add_debug_info("file error", err.to_string())
        })?;

        Ok(())
    }
}
