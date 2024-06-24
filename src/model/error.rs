use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum ServerError 
{
	//user
    UserNotFound,
    MailAlreadyInUse,

	//chat
	ChatNotFound,
    InvalidOwnerCount,

	//fallback
    UnexpectedError(String),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl IntoResponse for ServerError 
{
    fn into_response(self) -> Response 
    {
        println!("->> {:<12} - {self:?}", "INTO_RES");

		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		response.extensions_mut().insert(self);

		response
    }
}

impl ServerError 
{
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) 
    {
        #[allow(unreachable_patterns)]
		match self 
        {
            Self::MailAlreadyInUse 
            | Self::UserNotFound
			| Self::ChatNotFound
			| Self::InvalidOwnerCount => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),

            Self::UnexpectedError(_) => (StatusCode::BAD_REQUEST, ClientError::SERVICE_ERROR),

			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}


#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
	INVALID_PARAMS,
	SERVICE_ERROR,
}