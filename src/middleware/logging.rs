use std::sync::Arc;

use axum::extract::State;
use axum::http::{
    Method,
    Uri,
};
use axum::response::{
    IntoResponse,
    Response,
};
use axum::Json;
use serde_json::json;
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::middleware::auth::{
    self,
    Ctx,
};
use crate::middleware::cookies::Manager;
use crate::model::error;
use crate::model::log::{
    log_request,
    Repository,
    RequestLogLinePersonal,
};

pub async fn main_response_mapper(
    State(state): State<Arc<dyn Repository>>,
    uri: Uri,
    ctx: Option<Ctx>,
    req_method: Method,
    jar: Cookies,
    res: Response,
) -> Response
{
    let req_id = Uuid::now_v7();

    let service_error = res.extensions().get::<error::Server>();
    let client_status_error =
        service_error.map(error::Server::client_status_and_error);

    let error_response = client_status_error.as_ref().map(
        |(status_code, client_error, extra_info)| {
            let client_error_body = if extra_info.is_none()
            {
                json!(
                {
                    "error":
                    {
                        "req_id": req_id.to_string(),
                        "type": client_error.as_ref(),
                    }
                })
            }
            else
            {
                json!(
                {
                    "error":
                    {
                        "req_id": req_id.to_string(),
                        "type": client_error.as_ref(),
                        "extra": extra_info,
                    }
                })
            };

            (
                *status_code,
                Json(client_error_body),
            )
                .into_response()
        },
    );

    let client_error_option = client_status_error.map(|(_, client, _)| client);

    let device_id_option =
        jar.get_cookie(auth::CookieNames::DEVICE_ID.as_str()).ok();

    let user_info = RequestLogLinePersonal::new(
        ctx.map(Ctx::user_id),
        device_id_option,
    );

    log_request(
        state,
        req_id,
        user_info,
        req_method,
        uri,
        service_error,
        client_error_option,
    )
    .await;

    println!();

    error_response.unwrap_or(res)
}
