use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::model::user::UserFlag;

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
	ChatRequirementsInvalid,
	ChatDoesNotContainThisUser,
	InternalOwnersCountInvalid { expected: usize, found: usize },
	ChatInfoNotFound,

	//message
	MessageNotFound,
	MessageDoesNotContainThisChat,
	MessageDoesNotContainThisUser,

	//relation
	UserIsAlreadyFriend,
	UserIsAlreadyBlocked,
	UserYoureAddingIsBlocked,
	UserYoureAddingHasYouBlocked,
	UserYoureAddingNotFound,
	UserYoureAddingCantBeSelf,
	IncomingFriendRequestNotFound,

	//db
	FailedRead(String),
	FailedInsert(String),
	FailedUpdate(String),
	FailedDelete(String),
	TransactionError(String),
    InvalidID(String),
    FailedUserParsing,
    FailedChatParsing,
    FailedDateParsing,

	//auth
	AuthCtxNotInRequest,

	//cookies
	CookieNotFound(String),

	//auth - refresh token
	RefreshTokenNotFound,
	RefreshTokenDoesNotMatchDeviceId,

	//auth - acces token
	FailedCreatingAccesToken,
	AccesTokenHashKeyNotSet,
	AccesTokenInvalid,
	AccesTokenExpired,

	//hashing
	HashingPasswordFailed,
	VerifyingPasswordFailed,
	HashingPasswordFailedBlocking,
	VerifyingPasswordFailedBlocking,

	//permissions
	UserIsNotAdminOrOwner,
	IncorrectUserPermissions(UserFlag),

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
			//user
            Self::UserNotFound 
            | Self::UsernameAlreadyInUse 
            | Self::MailAlreadyInUse => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),


			//chat
			Self::ChatNotFound
			| Self::ChatAlreadyExists
			| Self::OwnerCountInvalid
			| Self::ChatRequirementsInvalid 
			| Self::ChatInfoNotFound  => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),
			Self::ChatDoesNotContainThisUser => (StatusCode::FORBIDDEN, ClientError::INVALID_PARAMS),


			//relation
			Self::UserYoureAddingNotFound
			| Self::UserYoureAddingCantBeSelf
			| Self::UserYoureAddingIsBlocked
			| Self::UserIsAlreadyFriend 
			| Self::UserIsAlreadyBlocked
			| Self::IncomingFriendRequestNotFound => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),


			//message
			Self::MessageNotFound
			| Self::MessageDoesNotContainThisChat
			| Self::MessageDoesNotContainThisUser => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),


			//auth
			Self::AccesTokenInvalid
			| Self::AccesTokenExpired
			| Self::RefreshTokenNotFound
			| Self::RefreshTokenDoesNotMatchDeviceId
			| Self::AuthCtxNotInRequest
			| Self::CookieNotFound(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
			Self::FailedCreatingAccesToken => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR),
	

			//db
			Self::FailedRead(_)
			| Self::FailedInsert(_)
			| Self::FailedUpdate(_)
			| Self::FailedDelete(_)
			| Self::TransactionError(_)
			| Self::UnexpectedError(_) => (StatusCode::BAD_REQUEST, ClientError::SERVICE_ERROR),
			Self::InvalidID(_) => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),

			//hashing
			Self::HashingPasswordFailed
			| Self::HashingPasswordFailedBlocking => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR),
			Self::VerifyingPasswordFailed
			| Self::VerifyingPasswordFailedBlocking => (StatusCode::FORBIDDEN, ClientError::INVALID_PARAMS),


			//permissions
			Self::UserIsNotAdminOrOwner
			| Self::IncorrectUserPermissions(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),


			//fallback
			Self::NotImplemented => (StatusCode::BAD_GATEWAY, ClientError::SERVICE_ERROR),
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