use axum::{http::{Method, Uri}, response::{IntoResponse, Response}, Json};
use serde_json::json;
use uuid::Uuid;

use crate::{middleware::Ctx, model::misc::{log_request, ServerError}};

pub async fn main_response_mapper(
	uri: Uri,
	ctx: Option<Ctx>,
	req_method: Method,
	res: Response,
) -> Response 
{
	let req_id = Uuid::now_v7();

	let service_error = res
        .extensions()
        .get::<ServerError>();
	let client_status_error = service_error
        .map(|se| se.client_status_and_error());

	let error_response =
		client_status_error
			.as_ref()
			.map(|(status_code, client_error)| {
				let client_error_body = json!({
					"error": {
                        "req_id": req_id.to_string(),
						"type": client_error.as_ref(),
					}
				});
        
				(*status_code, Json(client_error_body)).into_response()
			});
    
    let client_error = client_status_error.unzip().1;
    log_request(req_id, ctx, req_method, uri, service_error, client_error).await;

	println!();
	error_response.unwrap_or(res)
}