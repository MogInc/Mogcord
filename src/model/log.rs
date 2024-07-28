mod repository;

pub use repository::*;

use std::sync::Arc;
use axum::http::{Method, Uri};
use serde::{Serialize, Serializer};
use serde_json::json;
use uuid::Uuid;

use super::error;

pub async fn log_request(
	state: Arc<dyn Repository>,
	req_id: Uuid,
	user_info: RequestLogLinePersonal,
	req_method: Method,
	uri: Uri,
	service_error: Option<&error::Server<'_>>,
	client_error: Option<error::Client>,
)
{

    let timestamp = chrono::Utc::now();

	// Create the RequestLogLine
	let log_line = RequestLogLine 
	{
		req_id: req_id.to_string(),
		timestamp: timestamp.to_string(),

		user_info,

		req_path: uri.to_string(),
		req_method: req_method.to_string(),

		client_error_type: client_error.map(|e| e.as_ref().to_string()),
		server_error: service_error.cloned(),
	};


	println!("   ->> log_request: \n{:#}", json!(log_line));
	if let Err(err) = state.create_log(log_line).await
	{
		println!("	->> LOG INSERT FAILED");
		println!("   ->> log_request FAILED INSERT: \n{}", json!(err));
	}

}

#[derive(Debug, Serialize, Clone)]
pub struct RequestLogLinePersonal
{
	user_id: Option<String>,
	device_id: Option<String>,
}

impl RequestLogLinePersonal
{
	#[must_use]
	pub fn new(user_id: Option<String>, device_id: Option<String>) -> Self
	{
		Self
		{
			user_id,
			device_id,
		}
	}
}


#[derive(Debug)]
pub struct RequestLogLine<'err>
{
	pub req_id: String,      
	pub timestamp: String,
	
	//requesting user info
	pub user_info: RequestLogLinePersonal,

	// -- http request attributes.
	pub req_path: String,
	pub req_method: String,

	// -- Errors attributes.
	pub client_error_type: Option<String>,
	pub server_error: Option<error::Server<'err>>,
}

impl<'err> Serialize for RequestLogLine<'err> 
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("RequestLogLine", 7)?;
        state.serialize_field("req_id", &self.req_id)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("user_info", &self.user_info)?;
        state.serialize_field("req_path", &self.req_path)?;
        state.serialize_field("req_method", &self.req_method)?;
        if let Some(client_err) = &self.client_error_type
		{
			state.serialize_field("client_error_type", client_err)?;
			state.serialize_field("server_err", "see DB.")?;
		}
        state.end()
    }
}