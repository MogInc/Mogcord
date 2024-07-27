mod jwt;
mod ctx;
mod cookie_names;

pub use jwt::*;
pub use ctx::*;
pub use cookie_names::*;

pub const ACCES_TOKEN_TTL_MIN: i64 = 10 * 10000;
pub const REFRESH_TOKEN_TTL_MIN: i64 = 60 * 24 * 365;
pub const DEVICE_ID_TTL_MIN: i64 = 60 * 24 * 365 * 5;

use axum::{async_trait, body::Body, extract::FromRequestParts, http::{request::Parts, Request}, middleware::Next, response::Response};
use tower_cookies::Cookies;

use crate::model::error;
use crate::middleware::{auth, cookies::Manager};


pub async fn mw_require_authentication(
    ctx: Result<Ctx, error::Server<'_>>,
    req: Request<Body>, 
    next: Next
) -> Result<Response, error::Server>
{
    println!("AUTH MIDDLEWARE (REG): ");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_require_admin_authentication(
    ctx: Result<Ctx, error::Server<'_>>,
    req: Request<Body>, 
    next: Next
) -> Result<Response, error::Server>
{
    println!("AUTH MIDDLEWARE (MNG): ");

	match ctx
	{
		Ok(ctx) => 
		{
			if !&ctx.user_flag_ref().is_admin_or_owner()
			{
				return Err(
					error::Server::new(
						error::Kind::IncorrectValue, 
						error::OnType::Rights, 
						file!(), 
						line!()
					).add_client(error::Client::PERMISSION_NO_ADMIN)
				);
			}
		},
		Err(err) => return Err(err),
	}

    Ok(next.run(req).await)
}


pub async fn mw_ctx_resolver<'err>(
    jar: Cookies, 
    mut req: Request<Body>, 
    next: Next
) -> Result<Response, error::Server<'err>> 
{
	println!("MTX RESOLVER: ");

    let cookie_names_acces_token = auth::CookieNames::AUTH_ACCES;

	let ctx_result = match jar
		.get_cookie(cookie_names_acces_token.as_str())
		.and_then(|val| internal_parse_token(val.as_str()))
	{
		Ok(claims) => Ok(Ctx::new(claims.sub, claims.user_flag)),
		Err(e) => Err(e),
	};


	if ctx_result.is_err() && !matches!(ctx_result.as_ref().unwrap_err().kind, error::Kind::Expired)
	{
		jar.remove_cookie(cookie_names_acces_token.to_string());
	}

	req
        .extensions_mut()
        .insert(ctx_result);

	Ok(next.run(req).await)
}


#[async_trait]
impl<S> FromRequestParts<S> for Ctx where S: Send + Sync
{
    type Rejection = error::Server<'static>;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
		parts
			.extensions
			.get::<Result<Ctx, error::Server>>()
			.ok_or(error::Server::new(
				error::Kind::NotFound, 
				error::OnType::Ctx, 
				file!(),
				line!()
			))?
			.clone()
	}
}

fn internal_parse_token<'err>(acces_token: &str) -> Result<Claims, error::Server<'err>>
{
	let claims = jwt::extract_acces_token(acces_token, &TokenStatus::DisallowExpired)?;

    Ok(claims)
}