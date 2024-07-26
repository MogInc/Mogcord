use axum::{http::{Method, StatusCode, Uri}, response::{IntoResponse, Response}, Json};
use serde_json::json;
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::model::{error, log::{log_request, RequestLogLinePersonal}};
use crate::middleware::{auth::{self, Ctx}, cookies::Manager};

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
			.map(|(status_code, client_error, extra_info_option)| 
			{
				let client_error_body = json!(
				{
					"error": 
					{
                        "req_id": req_id.to_string(),
						"type": client_error.as_ref(),
						"type_info": client_error.as_str(),
						"extra": *extra_info_option.unwrap_or(&String::new()),
					}
				});
        
				(*status_code, Json(client_error_body)).into_response()
			});
    
    let client_error_option = client_status_error.unzip().1;

	let device_id_option = jar
		.get_cookie(auth::CookieNames::DEVICE_ID.as_str())
		.ok();

	let user_info = RequestLogLinePersonal::new(
		ctx.map(Ctx::user_id), device_id_option);

    log_request(req_id, user_info, req_method, uri, service_error, client_error_option).await;

	println!();
	
	error_response.unwrap_or({
		//shouldnt be able to panic
		if res.headers().get("content-length").unwrap() == "0"
		{
			(StatusCode::NO_CONTENT).into_response()
		}
		else
		{
			res
		}
	})
}