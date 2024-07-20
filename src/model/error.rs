use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use super::user;

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Server 
{
	//user
    UserNotFound,
    MailAlreadyInUse,
    UsernameAlreadyInUse,
	UserIsNotOwnerOfChat,

	//chat
	ChatNotFound,
	ChatAlreadyExists,
	ChatRequirementsInvalid,
	ChatDoesNotContainThisUser,
	OwnerCountInvalid { expected: usize, found: usize },
	ChatInfoNotFound,
	ChatNotAllowedToBeMade(ExtraInfo),
	ChatNotAllowedToGainUsers,
	ChatAlreadyHasThisUser,
	CantAddUsersToChatThatArentFriends,
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
	IncorrectUserPermissions{ expected_min_grade: user::Flag, found: user::Flag },

	//fallback
	NotImplemented,
    UnexpectedError(String),
}

#[derive(Debug, Clone, Serialize)]
pub enum ExtraInfo
{
	UserCreatingIsNotOwner,
}

impl fmt::Display for Server 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
        write!(f, "{self:?}")
    }
}

impl IntoResponse for Server 
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

impl Server 
{
	#[must_use]
	#[allow(clippy::match_same_arms)]
	pub fn client_status_and_error(&self) -> (StatusCode, Client) 
    {
        #[allow(unreachable_patterns)]
		match self 
        {
			//user
            Self::UserNotFound 
            | Self::UsernameAlreadyInUse 
            | Self::MailAlreadyInUse => (StatusCode::BAD_REQUEST, Client::INVALID_PARAMS),


			//chat
			Self::ChatNotFound
			| Self::ChatAlreadyExists
			| Self::OwnerCountInvalid {..}
			| Self::ChatRequirementsInvalid 
			| Self::ChatInfoNotFound => (StatusCode::BAD_REQUEST, Client::INVALID_PARAMS),
			Self::ChatDoesNotContainThisUser => (StatusCode::FORBIDDEN, Client::INVALID_PARAMS),


			//relation
			Self::UserYoureAddingNotFound
			| Self::UserYoureAddingCantBeSelf
			| Self::UserYoureAddingIsBlocked
			| Self::UserIsAlreadyFriend 
			| Self::UserIsAlreadyBlocked
			| Self::IncomingFriendRequestNotFound => (StatusCode::BAD_REQUEST, Client::INVALID_PARAMS),


			//message
			Self::MessageNotFound
			| Self::MessageDoesNotContainThisChat
			| Self::MessageDoesNotContainThisUser => (StatusCode::BAD_REQUEST, Client::INVALID_PARAMS),


			//auth
			Self::AccesTokenInvalid
			| Self::AccesTokenExpired
			| Self::RefreshTokenNotFound
			| Self::RefreshTokenDoesNotMatchDeviceId
			| Self::AuthCtxNotInRequest
			| Self::CookieNotFound(_) => (StatusCode::FORBIDDEN, Client::NO_AUTH),
			Self::FailedCreatingAccesToken => (StatusCode::INTERNAL_SERVER_ERROR, Client::SERVICE_ERROR),
	

			//db
			Self::FailedRead(_)
			| Self::FailedInsert(_)
			| Self::FailedUpdate(_)
			| Self::FailedDelete(_)
			| Self::TransactionError(_)
			| Self::UnexpectedError(_) => (StatusCode::BAD_REQUEST, Client::SERVICE_ERROR),
			Self::InvalidID(_) => (StatusCode::BAD_REQUEST, Client::INVALID_PARAMS),

			//hashing
			Self::HashingPasswordFailed
			| Self::HashingPasswordFailedBlocking => (StatusCode::INTERNAL_SERVER_ERROR, Client::SERVICE_ERROR),
			Self::VerifyingPasswordFailed
			| Self::VerifyingPasswordFailedBlocking => (StatusCode::FORBIDDEN, Client::INVALID_PARAMS),


			//permissions
			Self::UserIsNotAdminOrOwner
			| Self::IncorrectUserPermissions{..} => (StatusCode::FORBIDDEN, Client::NO_AUTH),


			//fallback
			Self::NotImplemented => (StatusCode::BAD_GATEWAY, Client::SERVICE_ERROR),
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				Client::SERVICE_ERROR,
			),
		}
	}
}


#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum Client
{
	INVALID_PARAMS,
	SERVICE_ERROR,
	NO_AUTH,
}