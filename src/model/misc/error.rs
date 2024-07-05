use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::middleware::cookies::AuthCookieNames;

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum ServerError 
{
	//user
    UserNotFound,
    MailAlreadyInUse,
    UsernameAlreadyInUse,

	//chat
	ChatNotFound,
	ChatAlreadyExists,
	OwnerCountInvalid,
	OwnersCountInvalid { expected: usize, found: usize },
	NameRequirementInvalid { expected: bool, found: bool },
	UsersRequirementInvalid { expected: bool, found: bool },
	ChatRequirementsInvalid,
	ChatDoesNotContainThisUser,

	//message
	MessageNotFound,
	ChatNotPartThisMessage,
	UserNotPartThisMessage,

	//db
	FailedRead(String),
	FailedInsert(String),
	FailedUpdate(String),
	FailedDelete(String),
	TransactionError(String),

	//auth
	AuthCtxNotInRequest,
	AuthCookieNotFound(AuthCookieNames),
	AuthCookieInvalid(AuthCookieNames),

	//jwt
	FailedCreatingToken,
	JWTKeyNotSet,
	JWTTokenInvalid,
	JWTTokenExpired,

	//hashing
	HashingPasswordFailed,
	VerifyingPasswordFailed,
	HashingPasswordFailedBlocking,
	VerifyingPasswordFailedBlocking,

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

		response
			.extensions_mut()
			.insert(self);

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
            | Self::UsernameAlreadyInUse 
			| Self::OwnerCountInvalid
            | Self::UserNotFound
			| Self::ChatNotFound
			| Self::MessageNotFound
			| Self::ChatAlreadyExists
			| Self::ChatNotPartThisMessage
			| Self::UserNotPartThisMessage
			| Self::ChatRequirementsInvalid => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),

			Self::ChatDoesNotContainThisUser => (StatusCode::FORBIDDEN, ClientError::INVALID_PARAMS),

			Self::NotImplemented => (StatusCode::BAD_GATEWAY, ClientError::SERVICE_ERROR),

			Self::AuthCtxNotInRequest
			| Self::AuthCookieNotFound(_)
			| Self::AuthCookieInvalid(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

			Self::FailedCreatingToken => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR),
			
			Self::HashingPasswordFailed
			| Self::HashingPasswordFailedBlocking => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR),
			Self::VerifyingPasswordFailed
			| Self::VerifyingPasswordFailedBlocking => (StatusCode::FORBIDDEN, ClientError::INVALID_PARAMS),

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
	NO_AUTH,
}