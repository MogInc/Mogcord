use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum ChatError 
{
    ChatNotFound,
    UnexpectedError(Option<String>),
}

impl fmt::Display for ChatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChatError::ChatNotFound => write!(f, "Chat not found"),

            ChatError::UnexpectedError(Some(err)) => 
            {
                println!("{}", err);
                write!(f, "Oopsie, unexpected error")
            },
            ChatError::UnexpectedError(None) => write!(f, "Oopsie, unexpected error"),
        }
    }
}

impl IntoResponse for ChatError 
{
    fn into_response(self) -> Response 
    {
        let status_code = match self 
        {
            ChatError::ChatNotFound => StatusCode::NOT_FOUND,
            ChatError::UnexpectedError(_) => StatusCode::BAD_REQUEST,
        };

        let body = Json(json!({ "error": self.to_string() }));

        (status_code, body).into_response()
    }
}
