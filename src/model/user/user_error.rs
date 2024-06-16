use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use derive_more::{Display};

#[derive(Debug, Display)]
pub enum UserError {
    UserNotFound,
    MailAlreadyInUse,
    UnexpectedError,
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let status_code = match self {
            UserError::MailAlreadyInUse => StatusCode::BAD_REQUEST,
            UserError::UserNotFound => StatusCode::NOT_FOUND,
            UserError::UnexpectedError => StatusCode::BAD_REQUEST,
        };

        let body = Json(json!({ "error": self.to_string() }));

        (status_code, body).into_response()
    }
}
