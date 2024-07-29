use std::sync::Arc;
use axum::{extract::State, response::IntoResponse};
use tower_cookies::Cookies;

use crate::model::{error, AppState};
use crate::middleware::{auth::{self, CreateAccesTokenRequest, TokenStatus}, cookies::Manager};
use crate::server_error;

pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    jar: Cookies
) -> impl IntoResponse
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
        .get_valid_token_by_device_id(&device_id_cookie)
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

    let create_token_request = CreateAccesTokenRequest::new(&claims.sub, &refresh_token.owner.flag);

    match auth::create_acces_token(&create_token_request)
    {
        Ok(token) => 
        {
            let updated_refresh_token = refresh_token.refresh_expiration()?;

            repo_refresh.update_expiration(&updated_refresh_token).await?;

            let cookie_names_acces = auth::CookieNames::AUTH_ACCES;
            let cookie_names_refresh = auth::CookieNames::AUTH_REFRESH;
            let cookie_names_device = auth::CookieNames::DEVICE_ID;

            jar.create_cookie(
                cookie_names_acces.to_string(), 
                token, 
                cookie_names_acces.ttl_in_mins(),
            );

            jar.create_cookie(
                cookie_names_refresh.to_string(), 
                updated_refresh_token.value, 
                cookie_names_refresh.ttl_in_mins(),
            );

            jar.create_cookie(
                cookie_names_device.to_string(), 
                updated_refresh_token.device_id, 
                cookie_names_device.ttl_in_mins(),
            );
            
            Ok(())
        },
        Err(err) => Err(err),
    }
}