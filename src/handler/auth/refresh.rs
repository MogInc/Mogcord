use std::sync::Arc;
use axum::{extract::State, response::IntoResponse};
use tower_cookies::Cookies;

use crate::model::{error, AppState};
use crate::middleware::{auth::{self, CreateAccesTokenRequest, TokenStatus}, cookies::Manager};

pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    jar: Cookies
) -> impl IntoResponse
{
    let repo_refresh = &state.refresh_tokens;

    let acces_token_cookie = jar.get_cookie(auth::CookieNames::AUTH_ACCES.as_str())
        .map_err(|err| error::Server::new_from_child(
            err, 
            error::Kind::NoAuth,
            error::OnType::Cookie,
            file!(), 
            line!()
        ))?;

    let claims = auth::extract_acces_token(&acces_token_cookie, &TokenStatus::AllowExpired)
        .map_err(|err| error::Server::new_from_child(
            err, 
            error::Kind::NoAuth,
            error::OnType::AccesToken,
            file!(), 
            line!()
        ))?;
   
    let refresh_token_cookie = jar.get_cookie(auth::CookieNames::AUTH_REFRESH.as_str())
        .map_err(|err| error::Server::new_from_child(
            err, 
            error::Kind::NoAuth,
            error::OnType::Cookie,
            file!(), 
            line!()
        ))?;

    let device_id_cookie = jar.get_cookie(auth::CookieNames::DEVICE_ID.as_str())
        .map_err(|err| error::Server::new_from_child(
            err, 
            error::Kind::NoAuth,
            error::OnType::Cookie,
            file!(), 
            line!()
        ))?;

    let refresh_token = repo_refresh
        .get_valid_token_by_device_id(&device_id_cookie)
        .await?;

    if !refresh_token.owner.flag.is_allowed_on_mogcord()
    {
        jar.remove_cookie(auth::CookieNames::AUTH_ACCES.to_string());
        jar.remove_cookie(auth::CookieNames::AUTH_REFRESH.to_string());
        return Err(error::Server::new(
            error::Kind::IncorrectPermissions,
            error::OnType::User,
            file!(),
            line!())
            .add_client(error::Client::NOT_ALLOWED_PLATFORM)
            .add_debug_info("user flag", refresh_token.owner.flag.to_string())
        );
    }

    if refresh_token.value != refresh_token_cookie
    {
        return Err(error::Server::new(
            error::Kind::NoAuth,
            error::OnType::RefreshToken,
            file!(),
            line!())
        );
    }

    let create_token_request = CreateAccesTokenRequest::new(&claims.sub, &refresh_token.owner.flag);

    match auth::create_acces_token(&create_token_request)
    {
        Ok(token) => 
        {
            let cookie_names_acces_token = auth::CookieNames::AUTH_ACCES;

            jar.create_cookie(
                cookie_names_acces_token.to_string(), 
                token, 
                cookie_names_acces_token.ttl_in_mins(),
            );
            
            Ok(())
        },
        Err(err) => Err(err),
    }
}