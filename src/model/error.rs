use std::{collections::HashMap, fmt};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub struct Server<'err>
{
	pub kind: Kind,
	pub on_type: OnType,
	pub stack: &'err str,
	pub line_nr: u32,
	pub debug_info: HashMap<&'err str, String>,
	pub pub_info: Option<String>,
	client: Option<Client>,
	pub child: Option<Box<Server<'err>>>,
}

impl<'err> Server<'err>
{
	#[must_use]
	pub fn new(
		kind: Kind,
		on_type: OnType,
		stack: &'err str,
		line_nr: u32,
	) -> Self
	{
		Self
		{
			kind,
			on_type,
			stack,
			line_nr,
			debug_info: HashMap::new(),
			pub_info: None,
			client: None,
			child: None,
		}
	}

	#[must_use]
	pub fn new_from_child(
		mut self,
		kind: Kind,
		on_type: OnType,
		stack: &'err str,
		line_nr: u32,
	) -> Self
	{
		Self
		{
			kind,
			on_type,
			stack,
			line_nr,
			debug_info: HashMap::new(),
			pub_info: self.pub_info.take(),
			client: self.client.take(),
			child: Some(Box::new(self)),
		}
	}

	#[must_use]
	pub fn from_child(
		mut self,
		stack: &'err str,
		line_nr: u32,
	) -> Self
	{
		Self
		{
			kind: self.kind.clone(),
			on_type: self.on_type.clone(),
			stack,
			line_nr,
			debug_info: HashMap::new(),
			pub_info: self.pub_info.take(),
			client: self.client.take(),
			child: Some(Box::new(self)),
		}
	}

	#[must_use]
	pub fn add_client(mut self, client: Client) -> Self
	{
		if self.client.is_none()
		{
			self.client = Some(client);
		}
		
		self
	}

	#[must_use]
	#[allow(clippy::extend_with_drain)]
	pub fn add_child(mut self, mut child: Self) -> Self
	{
		self.client = child.client.take();
		self.pub_info = child.pub_info.take();

		self.child = Some(Box::new(child));
		
		self
	}

	#[must_use]
	pub fn add_debug_info(mut self, key: &'err str, debug_info: String) -> Self
	{
		self.debug_info.insert(key, debug_info);

		self
	}

	#[must_use]
	pub fn add_public_info(mut self, public_info: String) -> Self
	{
		if self.pub_info.is_none()
		{
			self.pub_info = Some(public_info);
		}

		self
	}
}

#[derive(Debug, strum_macros::Display, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Kind
{
	AlreadyExists,
	AlreadyInUse,
	AlreadyPartOf,
	CantGainUsers,
	Create,
	Delete,
	Expired,
	Fetch,
	InValid,
	IncorrectPermissions,
	IncorrectValue,
	Insert,
	IsSelf,
	NoAuth,
	NoChange,
	NotAllowed,
	NotFound,
	NotImplemented,
	NotPartOf,
	Parse,
	Read,
	Revoke,
	Unexpected,
	Update,
	Verifying
}

#[derive(Debug, strum_macros::Display, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum OnType
{
	AccesToken,
	AccesTokenHashKey,
	Bucket,
	Channel,
	ChannelParent,
	Chat,
	ChatGroup,
	ChatPrivate,
	Cookie,
	Ctx,
	Date,
	Hashing,
	Log,
	Mail,
	Message,
	Mongo,
	RefreshToken,
	Relation,
	RelationBlocked,
	RelationFriend,
	Rights,
	Server,
	SpawnBlocking,
	Transaction,
	User,
	Username,
}

impl Server<'_>
{
    fn fmt_with_depth(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result 
	{
		write!(f, "{}: {:?}::{:?} - {} on ln:{} | {:?} |", depth, self.kind, self.on_type, self.stack, self.line_nr, self.debug_info)?;

		if let Some(ref child) = self.child 
		{
            write!(f, " -> ")?;
            child.fmt_with_depth(f, depth + 1)?;
        }

		Ok(())
    }
}

impl fmt::Display for Server<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
		self.fmt_with_depth(f, 0)
	}
}

impl IntoResponse for Server<'static>
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

impl Server<'_>
{
	#[must_use]
	#[allow(clippy::match_same_arms)]
	pub fn client_status_and_error(&self) -> (StatusCode, Client, Option<&String>) 
    {
		#[allow(clippy::match_wildcard_for_single_variants)]
		let status_code = match &self.kind
		{
			Kind::NotFound => StatusCode::NOT_FOUND,

			Kind::NoChange => StatusCode::NO_CONTENT,

			Kind::AlreadyExists
			| Kind::AlreadyInUse
			| Kind::AlreadyPartOf
			| Kind::CantGainUsers
			| Kind::IsSelf => StatusCode::CONFLICT,

			Kind::Expired
			| Kind::IncorrectPermissions
			| Kind::NotAllowed
			| Kind::NotPartOf
			| Kind::Verifying => StatusCode::FORBIDDEN,
			
			Kind::Create
			| Kind::Delete
			| Kind::Fetch
			| Kind::InValid
			| Kind::IncorrectValue 
			| Kind::Insert
			| Kind::Parse
			| Kind::Read
			| Kind::Revoke
			| Kind::Update => StatusCode::BAD_REQUEST,

			Kind::NotImplemented => StatusCode::NOT_IMPLEMENTED,

			Kind::NoAuth => StatusCode::UNAUTHORIZED,

			Kind::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
		};

		if let Some(client) = &self.client
		{
			(status_code, client.clone(), self.pub_info.as_ref())
		}
		else
		{
			(status_code, Client::SERVICE_ERROR, self.pub_info.as_ref())
		}
	}
}

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum Client
{
	CHAT_ALREADY_EXISTS,
	CHAT_CANT_GAIN_USERS,
	CHAT_ADD_NON_FRIEND,
	CHAT_ADD_WITH_SELF,
	INVALID_PARAMS,
	MAIL_IN_USE,
	MESSAGE_NOT_PART_CHANNEL,
	NOT_ALLOWED_PLATFORM,
	CHAT_EDIT_NOT_OWNER,
	CHAT_PARENT_CTX_NOT_PART_OF_PARENT,
	CHAT_CTX_NOT_PART_OF_CHAT,
	SERVER_CTX_NOT_PART_OF_SERVER,
	PERMISSION_NO_ADMIN,
	PERMISSION_NO_AUTH,
	PRIVATE_CHAT_TRY_EDIT,
	COOKIES_NOT_FOUND,
	RELATION_NO_INCOMING_FRIEND,
	MESSAGE_CREATE_FAIL,
	MESSAGE_EDIT_FAIL,
	RELATION_DUPLICATE_OUTGOING_FRIEND,
	SERVER_BLOCKED_YOU,
	SERVER_NOT_FOUND,
	SERVICE_ERROR,
	RELATION_SELF_TRY_BLOCK_SELF,
	RELATION_SELF_TRY_FRIEND_SELF,
	RELATION_SELF_TRY_UNBLOCK_SELF,
	RELATION_SELF_TRY_UNFRIEND_SELF,
	USERNAME_IN_USE,
	RELATION_USER_ALREADY_BLOCKED,
	RELATION_USER_ALREADY_FRIEND,
	RELATION_USER_BLOCKED,
	RELATION_USER_BLOCKED_YOU,
}

impl fmt::Display for Client 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
        write!(f, "{self:?}")
    }
}

#[must_use]
pub fn map_transaction<'err>(err: &mongodb::error::Error, file: &'err str, line: u32) -> Server<'err> 
{
    Server::new(
        Kind::Unexpected,
        OnType::Transaction,
        file,
        line,
    )
    .add_debug_info("mongo transaction error", err.to_string())
}