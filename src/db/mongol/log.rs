mod repository;

use std::collections::HashMap;
use serde::Serialize;

use crate::model::{error::{self}, log::{RequestLogLine, RequestLogLinePersonal}};

#[derive(Debug, Serialize)]
pub struct MongolLog
{
	req_id: String,      
	timestamp: String,
	user_info: RequestLogLinePersonal,
	req_path: String,
	req_method: String,
	client_error_type: Option<String>,
	server_error: Option<Vec<MongolLogServerError>>,
}

impl TryFrom<RequestLogLine<'_>> for MongolLog
{
    type Error = error::Server<'static>;

    fn try_from(value: RequestLogLine) -> Result<Self, Self::Error>
    {
        let test = create_server_error(value.server_error.as_ref());

        Ok(
            Self
            { 
                req_id: value.req_id,
                timestamp: value.timestamp,
                user_info: value.user_info,
                req_path: value.req_path,
                req_method: value.req_method,
                client_error_type: value.client_error_type,
                server_error: test,
            }
        )
    }
}

#[derive(Debug, Serialize)]
pub struct MongolLogServerError
{
	kind: String,
	on_type: String,
	stack: String,
	debug_info: HashMap<String, String>,
	pub_info: Option<String>,
}

fn create_server_error(value: Option<&error::Server<'_>>) -> Option<Vec<MongolLogServerError>>
{
    fn collect_errors(server: &error::Server<'_>, errors: &mut Vec<MongolLogServerError>) 
    {
        errors.push(
            MongolLogServerError 
            {
                kind: server.kind.to_string(),
                on_type: server.on_type.to_string(),
                stack: server.stack.to_string(),
                debug_info: server.debug_info.iter().map(|(key, val)| ((*key).to_string(), val.to_owned())).collect(),
                pub_info: server.pub_info.clone(),
            }
        );

        if let Some(child) = &server.child 
        {
            collect_errors(child, errors);
        }
    }

    value.map(|err_val| {
        let mut errors = Vec::new();
        collect_errors(err_val, &mut errors);
        errors
    })
}