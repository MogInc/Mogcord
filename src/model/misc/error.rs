use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum ServerError 
{
	//user
    UserNotFound,
    MailAlreadyInUse,

	//chat
	ChatNotFound,
	ChatAlreadyExists,
	InvalidOwnerCount,
	InvalidOwnersCount { expected: usize, found: usize },
	InvalidNameRequirement { expected: bool, found: bool },
	InvalidUsersRequirement { expected: bool, found: bool },
	InvalidChatRequirements,
	UserNotPartOfThisChat,

	//message
	MessageNotFound,

	//db
	FailedRead(String),
	FailedInsert(String),
	FailedUpdate(String),
	FailedDelete(String),
	TransactionError(String),

	//fallback
	NotImplemented,
    UnexpectedError(String),
}

impl fmt::Display for ServerError 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
        write!(f, "{self:?}")
    }
}

impl IntoResponse for ServerError 
{
    fn into_response(self) -> Response 
    {
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
			| Self::InvalidOwnerCount
            | Self::UserNotFound
			| Self::ChatNotFound
			| Self::MessageNotFound
			| Self::ChatAlreadyExists
			| Self::InvalidChatRequirements => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),

			Self::UserNotPartOfThisChat => (StatusCode::FORBIDDEN, ClientError::INVALID_PARAMS),

			Self::NotImplemented => (StatusCode::BAD_GATEWAY, ClientError::SERVICE_ERROR),
            
			Self::FailedRead(_)
			| Self::FailedInsert(_)
			| Self::FailedUpdate(_)
			| Self::FailedDelete(_)
			| Self::TransactionError(_)
			| Self::UnexpectedError(_) => (StatusCode::BAD_REQUEST, ClientError::SERVICE_ERROR),

			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}


#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError
{
	INVALID_PARAMS,
	SERVICE_ERROR,
}