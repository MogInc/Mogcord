use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::middleware::Ctx;

use super::error::{ClientError, ServerError};

pub async fn log_request(
	req_id: Uuid,
	ctx: Option<Ctx>,
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

		user_id: ctx.map(|x| x.user_id()),

		req_path: uri.to_string(),
		req_method: req_method.to_string(),

		client_error_type: client_error.map(|e| e.as_ref().to_string()),
		error_type,
		error_data,
	};

	println!("   ->> log_request: \n{}", json!(log_line));

    //TODO add saving to db or file
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine 
{
	req_id: String,      
	timestamp: String,
	
	//requesting user
	user_id: Option<String>,

	// -- http request attributes.
	req_path: String,
	req_method: String,

	// -- Errors attributes.
	client_error_type: Option<String>,
	error_type: Option<String>,
	error_data: Option<Value>,
}