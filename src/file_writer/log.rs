use std::path::Path;
use std::{fs::OpenOptions, io::BufWriter};
use std::io::Write;

use axum::async_trait;

use crate::{model::{error::{self}, log::{self, RequestLogLine}}, server_error};

use super::FileWriter;

#[async_trait]
impl<'a> log::Repository for FileWriter<'a>
{
    async fn create_log<'input, 'err>(&'input self, log: RequestLogLine<'input>) -> Result<(), error::Server<'err>>
    {
        let path = Path::new(self.folder_path)
            .join(format!("{}.log", chrono::offset::Local::now().date_naive()));

        //most likely too naive of a solution
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&path)
            .map_err(|err| 
                server_error!(error::Kind::FileOpening, error::OnType::Log)
                .add_debug_info("file error", err.to_string())
                .add_debug_info("path", format!("{path:?}"))
            )?;
        
        let mut buf_writer = BufWriter::new(file);
        
        let json = serde_json::to_string(&log)
            .map_err(|err| 
                server_error!(error::Kind::Parse, error::OnType::Log)
                .add_debug_info("file error", err.to_string())
            )?;
        
        writeln!(buf_writer, "{json}")
            .map_err(|err| 
                server_error!(error::Kind::Write, error::OnType::Log)
                .add_debug_info("file error", err.to_string())
            )?;
        
        buf_writer.flush()
            .map_err(|err| 
                server_error!(error::Kind::FlushBuffer, error::OnType::Log)
                .add_debug_info("file error", err.to_string())
            )?;
        
        Ok(())
    }
}