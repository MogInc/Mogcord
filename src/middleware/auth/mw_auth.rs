use axum::{async_trait, body::Body, extract::FromRequestParts, http::{request::Parts, Request}, middleware::Next, response::Response};
use tower_cookies::Cookies;

use crate::{middleware::cookies::{AuthCookieNames, CookieManager}, model::misc::ServerError};

use super::{jwt::{self, TokenStatus}, Ctx};


pub async fn mw_require_auth(
    ctx: Result<Ctx, ServerError>,
    req: Request<Body>, 
    next: Next
) -> Result<Response, ServerError>
{
    println!("AUTH MIDDLEWARE");

    ctx?;

    return Ok(next.run(req).await);
}


pub async fn mw_ctx_resolver(
    cookies: Cookies, 
    mut req: Request<Body>, 
    next: Next
) -> Result<Response, ServerError> 
{
	println!("IM HERE FOR SOME REASON");


    let auth_cookie_name = AuthCookieNames::AUTH_ACCES.into();

	let auth_token = CookieManager::get_cookie(&cookies, auth_cookie_name);


	let result_ctx = match auth_token
        .ok_or(ServerError::AuthCookieNotFound(AuthCookieNames::AUTH_ACCES))
		.and_then(|val| parse_token(val.as_str()))
	{
		Ok(user_id) => Ok(Ctx::new(user_id)),
		Err(e) => Err(e),
	};


	if result_ctx.is_err() && !matches!(result_ctx, Err(ServerError::AuthCookieNotFound(AuthCookieNames::AUTH_ACCES)))
	{
		CookieManager::remove_cookie(&cookies, auth_cookie_name);
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

fn parse_token(token: &str) -> Result<String, ServerError>
{
	let claims = jwt::extract_token(token, TokenStatus::DisallowExpired)?;

    return Ok(claims.sub());
}