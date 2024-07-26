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
	pub extra_public_info: Option<String>,
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

	pub fn new_from_child(
		self,
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
			debug_info: self.debug_info,
			extra_public_info: self.extra_public_info,
			client: self.client,
			child: self.child,
		}
	}

	pub fn from_child(
		self,
		stack: &'stack str,
		line_nr: u32,
	) -> Self
	{
		Self
		{
			kind: self.kind,
			on_type: self.on_type,
			stack,
			line_nr,
			debug_info: self.debug_info,
			extra_public_info: self.extra_public_info,
			client: self.client,
			child: self.child,
		}
	}

	#[must_use]
	pub fn add_client(mut self, client: Client) -> Self
	{
		self.client.get_or_insert(client);

		self
	}

	#[must_use]
	pub fn add_child(mut self, mut child: Self) -> Self
	{
		self.client = child.client.take();
		self.extra_public_info = child.extra_public_info.take();
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
		self.extra_public_info.insert(extra_info);

		self
	}
}

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Kind
{
	AlreadyInUse,
	AlreadyMember,
	CantGainUsers,
	Create,
	Delete,
	Fetch,
	Expired,
	InValid,
	IncorrectValue,
	IncorrectPermissions,
	Insert,
	IsSelf,
	NoAuth,
	NotAllowed,
	NotFound,
	NotImplemented,
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
	Transaction,
	RefreshToken,
	Relation,
	RelationFriend,
	RelationBlocked,
	Rights,
	Server,
	SpawnBlocking,
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
		write!(f, "{}: {:?}::{:?} - {} on ln:{} | {:?}", 0, self.kind, self.on_type, self.stack, self.line_nr, self.debug_info.join("-"))?;

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
			Kind::Expired
			| Kind::NotAllowed
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
	NO_ADMIN,
	NO_AUTH,
	NO_COOKIES,
	NO_MESSAGE_EDIT,
	NO_CHAT_PRIVATE_EDIT,
	MAIL_IN_USE,
	USER_ALREADY_BLOCKED,
	USER_BLOCKED_YOU,
	SERVER_BLOCKED_YOU,
	USERNAME_IN_USE,
	INVALID_PARAMS,
	NOT_ALLOWED_PLATFORM,
	NOT_PART_SERVER,
	NOT_PART_CHAT,
	SERVICE_ERROR,
	TRY_SELF_BLOCKED,
	TRY_SELF_FRIEND,
}

impl fmt::Display for Client 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
        write!(f, "{self}")
    }
}

impl Client
{
    #[must_use]
    pub fn as_str(&self) -> &str 
    {
        match self 
        {
            Client::NO_ADMIN => "Missing Admin Permissions, please refrain from using this endpoint",
			Client::NO_AUTH => "Missing authentication, please reauthorize",
			Client::NO_COOKIES => "Missing cookies",
			Client::NO_MESSAGE_EDIT => "Message cannot be edited",
			Client::NO_CHAT_PRIVATE_EDIT => "Private chat cannot be edited",
            Client::INVALID_PARAMS => "Invalid parameters",
            Client::MAIL_IN_USE => "Mail already in use",
            Client::USERNAME_IN_USE => "Username already in use",
            Client::NOT_ALLOWED_PLATFORM => "Your account has been suspended or disabled",
            Client::NOT_PART_CHAT => "Shoo shoo, youre not part of this chat",
            Client::NOT_PART_SERVER => "Shoo shoo, youre not part of this server",
            Client::USER_ALREADY_BLOCKED => "You have this already user blocked",
            Client::USER_BLOCKED_YOU => "This user has you blocked",
            Client::SERVER_BLOCKED_YOU => "Server owner has you blocked or you're on the server blocklist",
			Client::TRY_SELF_FRIEND => "Can't befriend yourself",
			Client::TRY_SELF_BLOCKED => "Can't block yourself",
            Client::SERVICE_ERROR => "",
        }
    }
}