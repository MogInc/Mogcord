use axum::{http::{Method, Uri}, response::{IntoResponse, Response}, Json};
use serde_json::json;
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::model::{error, log::{log_request, RequestLogLinePersonal}};
use crate::middleware::{auth::Ctx, cookies::{AuthCookieNames, Cookie2}};

pub async fn main_response_mapper(
	uri: Uri,
	ctx: Option<Ctx>,
	req_method: Method,
	jar: Cookies,
	res: Response,
) -> Response 
{
	let req_id = Uuid::now_v7();

	let service_error = res
        .extensions()
        .get::<error::Server>();
	let client_status_error = service_error
        .map(error::Server::client_status_and_error);

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
    
    let client_error_option = client_status_error.unzip().1;

	let device_id_option = jar
		.get_cookie(AuthCookieNames::DEVICE_ID.as_str())
		.ok();

	let user_info = RequestLogLinePersonal::new(
		ctx.map(Ctx::user_id), device_id_option);

    log_request(req_id, user_info, req_method, uri, service_error, client_error_option).await;

	println!();
	error_response.unwrap_or(res)
}