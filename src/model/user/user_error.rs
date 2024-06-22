use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum UserError 
{
    UserNotFound,
    MailAlreadyInUse,
    UnexpectedError(Option<String>),
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::UserNotFound => write!(f, "User not found"),
            UserError::MailAlreadyInUse => write!(f, "Mail already in use"),

            UserError::UnexpectedError(Some(err)) => 
            {
                println!("{}", err);
                write!(f, "Oopsie, unexpected error")
            },
            UserError::UnexpectedError(None) => write!(f, "Oopsie, unexpected error"),
        }
    }
}

impl IntoResponse for UserError 
{
    fn into_response(self) -> Response 
    {
        let status_code = match self 
        {
            UserError::MailAlreadyInUse => StatusCode::BAD_REQUEST,
            UserError::UserNotFound => StatusCode::NOT_FOUND,
            UserError::UnexpectedError(_) => StatusCode::BAD_REQUEST,
        };

        let body = Json(json!({ "error": self.to_string() }));

        (status_code, body).into_response()
    }
}
