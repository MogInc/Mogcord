use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use super::error::{ClientError, ServerError};

pub async fn log_request(
	req_id: Uuid,
	user_info: RequestLogLinePersonal,
	req_method: Method,
	uri: Uri,
	service_error: Option<&ServerError>,
	client_error: Option<ClientError>,
) {

    let timestamp = chrono::Utc::now();

	let error_type = service_error.map(|se| se.to_string());
	let error_data = serde_json::to_value(service_error)
		.ok()
		.and_then(|mut v| v.get_mut("data").map(|v| v.take()));

	// Create the RequestLogLine
	let log_line = RequestLogLine {
		req_id: req_id.to_string(),
		timestamp: timestamp.to_string(),

		user_info: user_info,

		req_path: uri.to_string(),
		req_method: req_method.to_string(),

		client_error_type: client_error.map(|e| e.as_ref().to_string()),
		error_type,
		error_data,
	};

	println!("   ->> log_request: \n{}", json!(log_line));

    //TODO add saving to db or file
}

#[derive(Serialize)]
pub struct RequestLogLinePersonal
{
	user_id: Option<String>,
	device_id: Option<String>,
}

impl RequestLogLinePersonal
{
	pub fn new(user_id: Option<String>, device_id: Option<String>) -> Self
	{
		Self
		{
			user_id: user_id,
			device_id: device_id,
		}
	}
}


#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine 
{
	req_id: String,      
	timestamp: String,
	
	//requesting user info
	user_info: RequestLogLinePersonal,

	// -- http request attributes.
	req_path: String,
	req_method: String,

	// -- Errors attributes.
	client_error_type: Option<String>,
	error_type: Option<String>,
	error_data: Option<Value>,
}