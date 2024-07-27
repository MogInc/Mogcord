use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub struct Server<'stack>
{
	pub kind: Kind,
	pub on_type: OnType,
	stack: &'stack str,
	line_nr: u32,
	debug_info: Vec<String>,
	extra_public_info: Option<String>,
	client: Option<Client>,
	child: Option<Box<Server<'stack>>>,
}

impl<'stack> Server<'stack>
{
	#[must_use]
	pub fn new(
		kind: Kind,
		on_type: OnType,
		stack: &'stack str,
		line_nr: u32,
	) -> Self
	{
		Self
		{
			kind,
			on_type,
			stack,
			line_nr,
			debug_info: Vec::new(),
			extra_public_info: None,
			client: None,
			child: None,
		}
	}

	#[must_use]
	pub fn new_from_child(
		mut self,
		kind: Kind,
		on_type: OnType,
		stack: &'stack str,
		line_nr: u32,
	) -> Self
	{
		Self
		{
			kind,
			on_type,
			stack,
			line_nr,
			debug_info: self.debug_info.drain(..).collect(),
			extra_public_info: self.extra_public_info.take(),
			client: self.client.take(),
			child: Some(Box::new(self)),
		}
	}

	#[must_use]
	pub fn from_child(
		mut self,
		stack: &'stack str,
		line_nr: u32,
	) -> Self
	{
		Self
		{
			kind: self.kind.clone(),
			on_type: self.on_type.clone(),
			stack,
			line_nr,
			debug_info: self.debug_info.drain(..).collect(),
			extra_public_info: self.extra_public_info.take(),
			client: self.client.take(),
			child: Some(Box::new(self)),
		}
	}

	#[must_use]
	pub fn add_client(mut self, client: Client) -> Self
	{
		self.client.get_or_insert(client);

		self
	}

	#[must_use]
	#[allow(clippy::extend_with_drain)]
	pub fn add_child(mut self, mut child: Self) -> Self
	{
		self.client = child.client.take();
		self.extra_public_info = child.extra_public_info.take();
		//i want child vec to pop itself
		self.debug_info.extend(child.debug_info.drain(..));

		self.child = Some(Box::new(child));
		
		self
	}

	#[must_use]
	pub fn add_debug_info(mut self, extra_info: String) -> Self
	{
		self.debug_info.push(extra_info);

		self
	}

	#[must_use]
	pub fn expose_public_extra_info(mut self, extra_info: String) -> Self
	{
		if self.client.is_none()
		{
			let _ = self.extra_public_info.insert(extra_info);
		}

		self
	}
}

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Kind
{
	AlreadyExists,
	AlreadyInUse,
	AlreadyMember,
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
	Unexpected,
	Update,
	Verifying
}

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
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
        write!(f, "{}: {:?}::{:?} - {} on ln:{}", depth, self.kind, self.on_type, self.stack, self.line_nr)?;

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
		write!(f, "{}: {:?}::{:?} - {} on ln:{} | {}", 0, self.kind, self.on_type, self.stack, self.line_nr, self.debug_info.join("-"))?;

		if let Some(ref child) = self.child 
		{
            write!(f, " -> ")?;
            child.fmt_with_depth(f, 1)?;
        }

		Ok(())
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
			Kind::AlreadyExists => StatusCode::CONFLICT,
			Kind::Expired
			| Kind::NotAllowed
			| Kind::NotPartOf
			| Kind::IncorrectPermissions => StatusCode::FORBIDDEN,
			Kind::IncorrectValue 
			| Kind::InValid => StatusCode::BAD_REQUEST,
			Kind::NotImplemented => StatusCode::NOT_IMPLEMENTED,
			Kind::NoAuth => StatusCode::UNAUTHORIZED,
			_ => StatusCode::INTERNAL_SERVER_ERROR,
		};

		if let Some(client) = &self.client
		{
			(status_code, client.clone(), self.extra_public_info.as_ref())
		}
		else
		{
			(status_code, Client::SERVICE_ERROR, self.extra_public_info.as_ref())
		}
	}
}

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum Client
{
	CHAT_ALREADY_EXISTS,
	CHAT_CANT_GAIN_USERS,
	CHAT_NO_FRIEND,
	CHAT_WITH_SELF,
	INVALID_PARAMS,
	MAIL_IN_USE,
	MESSAGE_NOT_PART_CHANNEL,
	NOT_ALLOWED_PLATFORM,
	NOT_OWNER_CHAT,
	NOT_PART_CHANNEL_PARENT,
	NOT_PART_CHAT,
	NOT_PART_SERVER,
	NO_ADMIN,
	NO_AUTH,
	NO_CHAT_PRIVATE_EDIT,
	NO_COOKIES,
	NO_INCOMING_FRIEND,
	NO_MESSAGE_CREATE,
	NO_MESSAGE_EDIT,
	OUTGOING_FRIEND,
	SERVER_BLOCKED_YOU,
	SERVER_NOT_FOUND,
	SERVICE_ERROR,
	TRY_ADD_SELF_BLOCKED,
	TRY_ADD_SELF_FRIEND,
	TRY_REMOVE_SELF_BLOCKED,
	TRY_REMOVE_SELF_FRIEND,
	USERNAME_IN_USE,
	USER_ALREADY_BLOCKED,
	USER_ALREADY_FRIEND,
	USER_BLOCKED,
	USER_BLOCKED_YOU,
}

impl fmt::Display for Client 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
        write!(f, "{self:?}")
    }
}

impl Client
{
    #[must_use]
    pub fn as_str(&self) -> &str 
    {
        match self 
        {
			Client::NO_AUTH => "Missing authentication, please reauthorize",
			Client::NO_CHAT_PRIVATE_EDIT => "Private chat cannot be edited",
			Client::NO_COOKIES => "Missing cookies",
			Client::NO_INCOMING_FRIEND => "You can't confirm a friendship that doesn't exist",
			Client::NO_MESSAGE_CREATE => "Message cannot be created",
			Client::NO_MESSAGE_EDIT => "Message cannot be edited",
			Client::OUTGOING_FRIEND => "You already have a friend request outgoing or youre already friends",
			Client::SERVER_NOT_FOUND => "Server not found",
			Client::TRY_ADD_SELF_BLOCKED => "Can't block yourself",
			Client::TRY_ADD_SELF_FRIEND => "Can't befriend yourself",
			Client::TRY_REMOVE_SELF_BLOCKED => "Can't unfriend yourself",
			Client::TRY_REMOVE_SELF_FRIEND => "Can't unblock yourself",
			Client::USER_ALREADY_FRIEND => "User is already your friend",
            Client::CHAT_ALREADY_EXISTS => "Chat already exists",
            Client::CHAT_CANT_GAIN_USERS => "Chat cant gain users",
            Client::CHAT_NO_FRIEND => "Can't have chat with non friends, try making a server",
            Client::CHAT_WITH_SELF => "Can't have chat with yourself",
            Client::INVALID_PARAMS => "Invalid parameters",
            Client::MAIL_IN_USE => "Mail already in use",
            Client::MESSAGE_NOT_PART_CHANNEL => "Message doesnt belong to this channel",
            Client::NOT_ALLOWED_PLATFORM => "Your account has been suspended or disabled",
            Client::NOT_OWNER_CHAT => "You're not owner of the chat",
            Client::NOT_PART_CHANNEL_PARENT => "Shoo shoo, youre not part of this chat or server",
            Client::NOT_PART_CHAT => "Shoo shoo, youre not part of this chat",
            Client::NOT_PART_SERVER => "Shoo shoo, youre not part of this server",
            Client::NO_ADMIN => "Missing Admin Permissions, please refrain from using this endpoint",
            Client::SERVER_BLOCKED_YOU => "Server owner has you blocked or you're on the server blocklist",
            Client::SERVICE_ERROR => "eh oh...",
            Client::USERNAME_IN_USE => "Username already in use",
            Client::USER_ALREADY_BLOCKED => "You have already this user blocked",
            Client::USER_BLOCKED => "You have this user blocked",
            Client::USER_BLOCKED_YOU => "This user has you blocked",
        }
    }
}

#[must_use]
pub fn map_transaction<'stack>(err: &mongodb::error::Error, file: &'stack str, line: u32) -> Server<'stack> 
{
    Server::new(
        Kind::Unexpected,
        OnType::Transaction,
        file,
        line,
    )
    .add_debug_info(err.to_string())
}