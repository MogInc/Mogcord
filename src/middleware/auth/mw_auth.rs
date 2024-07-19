use axum::{async_trait, body::Body, extract::FromRequestParts, http::{request::Parts, Request}, middleware::Next, response::Response};
use tower_cookies::Cookies;

use crate::{middleware::cookies::{AuthCookieNames, Cookie2}, model::error};

use super::{jwt::{self, Claims, TokenStatus}, Ctx};


pub async fn mw_require_regular_auth(
    ctx: Result<Ctx, error::Server>,
    req: Request<Body>, 
    next: Next
) -> Result<Response, error::Server>
{
    println!("AUTH MIDDLEWARE (REG): ");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_require_admin_auth(
    ctx: Result<Ctx, error::Server>,
    req: Request<Body>, 
    next: Next
) -> Result<Response, error::Server>
{
    println!("AUTH MIDDLEWARE (MNG): ");

	match ctx
	{
		Ok(ctx) => 
		{
			if !&ctx.user_flag().is_admin_or_owner()
			{
				return Err(error::Server::UserIsNotAdminOrOwner);
			}
		},
		Err(err) => return Err(err),
	}

    Ok(next.run(req).await)
}


pub async fn mw_ctx_resolver(
    jar: Cookies, 
    mut req: Request<Body>, 
    next: Next
) -> Result<Response, error::Server> 
{
	println!("MTX RESOLVER: ");

    let cookie_names_acces_token = AuthCookieNames::AUTH_ACCES;

	let ctx_result = match jar
		.get_cookie(cookie_names_acces_token.as_str())
		.and_then(|val| internal_parse_token(val.as_str()))
	{
		Ok(claims) => Ok(Ctx::new(claims.sub, claims.user_flag)),
		Err(e) => Err(e),
	};


	if ctx_result.is_err() && !matches!(ctx_result, Err(error::Server::AccesTokenExpired))
	{
		jar.remove_cookie(cookie_names_acces_token.to_string());
	}

	req
        .extensions_mut()
        .insert(ctx_result);

	Ok(next.run(req).await)
}


#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx
{
    type Rejection = error::Server;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, error::Server> {
		parts
			.extensions
			.get::<Result<Ctx, error::Server>>()
			.ok_or(error::Server::AuthCtxNotInRequest)?
			.clone()
	}
}

fn internal_parse_token(acces_token: &str) -> Result<Claims, error::Server>
{
	let claims = jwt::extract_acces_token(acces_token, &TokenStatus::DisallowExpired)?;

    Ok(claims)
}