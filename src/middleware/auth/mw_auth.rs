use axum::{async_trait, body::Body, extract::FromRequestParts, http::{request::Parts, Request}, middleware::Next, response::Response};
use tower_cookies::Cookies;

use crate::{middleware::cookies::{AuthCookieNames, Cookie2}, model::misc::ServerError};

use super::{jwt::{self, Claims, TokenStatus}, Ctx};


pub async fn mw_require_regular_auth(
    ctx: Result<Ctx, ServerError>,
    req: Request<Body>, 
    next: Next
) -> Result<Response, ServerError>
{
    println!("AUTH MIDDLEWARE (REG): ");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_require_management_auth(
    ctx: Result<Ctx, ServerError>,
    req: Request<Body>, 
    next: Next
) -> Result<Response, ServerError>
{
    println!("AUTH MIDDLEWARE (MNG): ");

	match ctx
	{
		Ok(ctx) => 
		{
			if !ctx.user_flag_ref().is_admin_or_owner()
			{
				return Err(ServerError::UserIsNotAdminOrOwner);
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
) -> Result<Response, ServerError> 
{
	println!("MTX RESOLVER: ");

    let cookie_names_acces_token = AuthCookieNames::AUTH_ACCES;

	let acces_token_options = jar.get_cookie(cookie_names_acces_token.as_str());


	let result_ctx = match acces_token_options
        .ok_or(ServerError::AuthCookieNotFound(AuthCookieNames::AUTH_ACCES))
		.and_then(|val| parse_token(val.as_str()))
	{
		Ok(claims) => Ok(Ctx::new(claims.sub, claims.user_flag)),
		Err(e) => Err(e),
	};


	if result_ctx.is_err() && !matches!(result_ctx, Err(ServerError::AccesTokenExpired))
	{
		jar.remove_cookie(cookie_names_acces_token.as_str());
	}

	req
        .extensions_mut()
        .insert(result_ctx);

	Ok(next.run(req).await)
}


#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx
{
    type Rejection = ServerError;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, ServerError> {
		parts
			.extensions
			.get::<Result<Ctx, ServerError>>()
			.ok_or(ServerError::AuthCtxNotInRequest)?
			.clone()
	}
}

fn parse_token(token: &str) -> Result<Claims, ServerError>
{
	let claims = jwt::extract_acces_token(token, TokenStatus::DisallowExpired)?;

    Ok(claims)
}