use std::sync::Arc;
use tower_cookies::Cookies;

use crate::handlers::logic;
use crate::model::{error, AppState};
use crate::middleware::{auth::{self, TokenStatus}, cookies::Manager};
use crate::server_error;

pub async fn refresh_token<'err>(
    state: &Arc<AppState>,
    jar: &Cookies
) -> error::Result<'err, ()>
{
    let repo_refresh = &state.refresh_tokens;

    let acces_token_cookie = jar.get_cookie(auth::CookieNames::AUTH_ACCES.as_str())
        .map_err(|err| server_error!(err, error::Kind::NoAuth, error::OnType::Cookie))?;

    let claims = auth::extract_acces_token(&acces_token_cookie, &TokenStatus::AllowExpired)
        .map_err(|err| server_error!(err, error::Kind::NoAuth, error::OnType::AccesToken))?;
   
    let refresh_token_cookie = jar.get_cookie(auth::CookieNames::AUTH_REFRESH.as_str())
        .map_err(|err| server_error!(err, error::Kind::NoAuth, error::OnType::Cookie))?;

    let device_id_cookie = jar.get_cookie(auth::CookieNames::DEVICE_ID.as_str())
        .map_err(|err| server_error!(err, error::Kind::NoAuth, error::OnType::Cookie))?;

    let refresh_token = repo_refresh
        .get_valid_token(&device_id_cookie, &claims.sub)
        .await?;

    if !refresh_token.owner.flag.is_allowed_on_mogcord()
    {
        jar.remove_cookie(auth::CookieNames::AUTH_ACCES.to_string());
        jar.remove_cookie(auth::CookieNames::AUTH_REFRESH.to_string());
        return Err(server_error!(error::Kind::IncorrectPermissions, error::OnType::User)
            .add_client(error::Client::NOT_ALLOWED_PLATFORM)
            .add_debug_info("user flag", refresh_token.owner.flag.to_string())
        );
    }

    if refresh_token.value != refresh_token_cookie
    {
        return Err(server_error!(error::Kind::NoAuth, error::OnType::RefreshToken));
    }

    let updated_refresh_token = refresh_token.refresh_expiration()?;

    logic::auth::cookies::create_auth_cookies(jar, updated_refresh_token)
}