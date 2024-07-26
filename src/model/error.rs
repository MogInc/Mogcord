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
	ServerDoesNotContainThisUser,
	OwnerCountInvalid { expected: usize, found: usize },
	UserCountInvalid { min: usize, found: usize },
	ChatInfoNotFound,
	ChatNotAllowedToBeMade(ExtraInfo),
	ChatNotAllowedToGainUsers,
	ChatAlreadyHasThisUser,
	ServerAlreadyHasThisUser,
	CantAddUsersToChatThatArentFriends,
	CantAddOwnerAsUser,
	ChannelNotFound,
	ChannelNotPassed,
	NotAllowedToMakeAMessageInThisChannel,
	CantUpdatePrivateChat,
	ServerNotFound,
	//message
	MessageNotFound,
	MessageDoesNotContainThisChat,
	MessageDoesNotContainThisUser,
	FailedToAddUserToServer,
	MessageNotAllowedToBeEdited,
	NotAllowedToRetrieveMessages,

	//relation
	UserIsAlreadyFriend,
	UserIsAlreadyBlocked,
	UserYoureAddingIsBlocked,
	UserYoureAddingHasYouBlocked,
	ServerOwnerHasYouBlocked,
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
	CantHaveChatWithSelf,
	OutgoingUserNotFriend,
}


#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub struct Server2<'stack>
{
	kind: Kind,
	on_type: OnType,
	stack: &'stack str,
	client: Option<Client>,
	child: Option<Box<Server2<'stack>>>,
}

impl<'stack> Server2<'stack>
{
	#[must_use]
	pub fn new_with_client_and_child(
		kind: Kind,
		on_type: OnType,
		stack: &'stack str,
		client: Client,
		child: Self,
	) -> Self
	{
		Self
		{
			kind,
			on_type,
			stack,
			client: Some(client),
			child: Some(Box::new(child)),
		}
	}

	#[must_use]
	pub fn new_with_client(
		kind: Kind,
		on_type: OnType,
		stack: &'stack str,
		client: Client,
	) -> Self
	{
		Self
		{
			kind,
			on_type,
			stack,
			client: Some(client),
			child: None,
		}
	}

	#[must_use]
	pub fn new_with_child(
		kind: Kind,
		on_type: OnType,
		stack: &'stack str,
		child: Self,
	) -> Self
	{
		Self
		{
			kind,
			on_type,
			stack,
			client: None,
			child: Some(Box::new(child)),
		}
	}

	#[must_use]
	pub fn new(
		kind: Kind,
		on_type: OnType,
		stack: &'stack str
	) -> Self
	{
		Self
		{
			kind,
			on_type,
			stack,
			client: None,
			child: None,
		}
	}
}

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Kind
{
   NotFound,
   NotImplemented,
   UnexpectedError(String),
}

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum OnType
{
   Chat,
   Message,
}

impl Server2<'_> 
{
    fn fmt_with_depth(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result 
	{
        write!(f, "{}: {:?}::{:?} - {:?}", depth, self.kind, self.on_type, self.stack)?;

		if let Some(ref child) = self.child 
		{
            write!(f, " -> ")?;
            child.fmt_with_depth(f, depth + 1)?;
        }

        Ok(())
    }
}

impl fmt::Display for Server2<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
        self.fmt_with_depth(f, 0)
	}
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

impl IntoResponse for Server2<'static>
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

impl Server2<'_>
{
	#[must_use]
	#[allow(clippy::match_same_arms)]
	pub fn client_status_and_error(&self) -> (StatusCode, Client) 
    {
		#[allow(clippy::match_wildcard_for_single_variants)]
		let status_code = match &self.kind
		{
			Kind::NotFound => StatusCode::NOT_FOUND,
			Kind::NotImplemented => StatusCode::NOT_IMPLEMENTED,
			_ => StatusCode::INTERNAL_SERVER_ERROR,
		};

		if let Some(client) = &self.client
		{
			(status_code, client.clone())
		}
		else
		{
			(status_code, Client::SERVICE_ERROR)
		}
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


#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum Client
{
	INVALID_PARAMS,
	NO_AUTH,
	SERVICE_ERROR,
}

impl fmt::Display for Client 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
        write!(f, "{}", self.as_str())
    }
}

impl Client
{
    #[must_use]
    pub fn as_str(&self) -> &str 
    {
        match self 
        {
            Client::INVALID_PARAMS => "ACCES_TOKEN",
            Client::NO_AUTH => "NO_AUTH",
            Client::SERVICE_ERROR => "SERVICE_ERROR",
        }
    }
}