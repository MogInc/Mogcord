mod cookie_names;
mod ctx;
mod jwt;

use std::sync::Arc;

pub use cookie_names::*;
pub use ctx::*;
pub use jwt::*;

pub const ACCES_TOKEN_TTL_MIN: i64 = 10 * 10000;
pub const REFRESH_TOKEN_TTL_MIN: i64 = 60 * 24 * 365;
pub const DEVICE_ID_TTL_MIN: i64 = 60 * 24 * 365 * 5;

use axum::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;

use crate::middleware::auth;
use crate::middleware::cookies::Manager;
use crate::model::{error, AppState};
use crate::server_error;

pub async fn mw_require_authentication(
    ctx: error::Result<'_, Ctx>,
    req: Request<Body>,
    next: Next,
) -> error::Result<Response>
{
    println!("AUTH MIDDLEWARE (REG): ");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_require_admin_authentication(
    ctx: error::Result<'_, Ctx>,
    req: Request<Body>,
    next: Next,
) -> error::Result<Response>
{
    println!("AUTH MIDDLEWARE (MNG): ");

    match ctx
    {
        Ok(ctx) =>
        {
            if !&ctx.is_admin()
            {
                return Err(server_error!(error::Kind::NoAuth, error::OnType::Rights)
                    .add_client(error::Client::PERMISSION_NO_ADMIN));
            }
        },
        Err(err) => return Err(err),
    }

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver<'err>(
    State(state): State<Arc<AppState>>,
    jar: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> error::Result<'err, Response>
{
    println!("MTX RESOLVER: ");

    let mut ctx_result = get_ctx(&jar);

    match ctx_result
    {
        Ok(_) => (),
        Err(err) if matches!(err.kind, error::Kind::Expired) =>
        {
            println!("REFRESH");

            let _ = crate::handlers::logic::auth::refresh_token(&state, &jar).await;
            ctx_result = get_ctx(&jar);
        },
        Err(_) => jar.remove_cookie(auth::CookieNames::AUTH_ACCES.to_string()),
    }

    req.extensions_mut().insert(ctx_result);

    Ok(next.run(req).await)
}

pub fn get_ctx<'err>(jar: &Cookies) -> Result<Ctx, error::Server<'err>>
{
    match jar
        .get_cookie(auth::CookieNames::AUTH_ACCES.as_str())
        .and_then(|val| internal_parse_token(val.as_str()))
    {
        Ok(claims) => Ok(Ctx::new(claims.sub, claims.is_admin)),
        Err(e) => Err(e),
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Ctx
where
    S: Send + Sync,
{
    type Rejection = error::Server<'static>;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection>
    {
        parts
            .extensions
            .get::<Result<Ctx, error::Server>>()
            .ok_or(server_error!(error::Kind::NotFound, error::OnType::Ctx))?
            .clone()
    }
}

fn internal_parse_token<'err>(acces_token: &str) -> error::Result<'err, Claims>
{
    let claims = jwt::extract_acces_token(acces_token, &TokenStatus::DisallowExpired)?;

    Ok(claims)
}
