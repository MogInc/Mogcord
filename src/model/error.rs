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
	debug_info: Option<String>,
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
			debug_info: None,
			extra_public_info: None,
			client: None,
			child: None,
		}
	}

	pub fn new_from_child(
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

		self.child = Some(Box::new(child));
		
		self
	}

	#[must_use]
	pub fn add_debug_info(mut self, extra_info: String) -> Self
	{
		self.debug_info.insert(extra_info);

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
	Auth,
	Channel,
	ChannelParent,
	ChatPrivate,
	ChatGroup,
	Cookie,
	Ctx,
	Date,
	Hashing,
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
}

impl Server<'_>
{
    fn fmt_with_depth(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result 
	{
        write!(f, "{}: {:?}::{:?} - {} on ln:{} | {:?}", depth, self.kind, self.on_type, self.stack, self.line_nr, self.debug_info)?;

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
		let mut status_code = match &self.kind
		{
			Kind::NotFound => StatusCode::NOT_FOUND,
			Kind::Expired
			| Kind::IncorrectPermissions => StatusCode::FORBIDDEN,
			Kind::IncorrectValue 
			| Kind::InValid => StatusCode::BAD_REQUEST,
			Kind::NotImplemented => StatusCode::NOT_IMPLEMENTED,
			_ => StatusCode::INTERNAL_SERVER_ERROR,
		};

		if let OnType::Auth = &self.on_type
		{
			status_code = StatusCode::UNAUTHORIZED;
		}

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
	INVALID_PARAMS,
	SERVICE_ERROR,
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
            Client::NO_ADMIN => "Missing Admin Permissions, please refrain from using this endpoint.",
			Client::NO_AUTH => "Missing authentication, please reauthorize.",
			Client::NO_COOKIES => "Missing cookies.",
			Client::NO_MESSAGE_EDIT => "Message cannot be edited",
            Client::INVALID_PARAMS => "Invalid parameters",
            Client::SERVICE_ERROR => "",
        }
    }
}