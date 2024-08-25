use std::sync::Arc;

use tower_cookies::Cookies;

use crate::middleware::auth::{self, CreateAccesTokenRequest};
use crate::middleware::cookies::Manager;
use crate::model::refresh_token::RefreshToken;
use crate::model::user::User;
use crate::model::{error, AppState};

pub fn create_auth_cookies<'err>(
    jar: &Cookies,
    refresh_token: RefreshToken,
) -> error::Result<'err, ()>
{
    let user = refresh_token.owner;
    let create_token_request = CreateAccesTokenRequest::new(
        &user.id,
        user.flag.is_mogcord_admin_or_owner(),
    );

    match auth::create_acces_token(&create_token_request)
    {
        Ok(acces_token) =>
        {
            let cookie_names_acces_token = auth::CookieNames::AUTH_ACCES;
            let cookie_names_refresh_token = auth::CookieNames::AUTH_REFRESH;
            let cookie_names_device_id = auth::CookieNames::DEVICE_ID;

            jar.create_cookie(
                cookie_names_acces_token.to_string(),
                acces_token,
                cookie_names_acces_token.ttl_in_mins(),
            );

            jar.create_cookie(
                cookie_names_refresh_token.to_string(),
                refresh_token.value,
                cookie_names_refresh_token.ttl_in_mins(),
            );

            jar.create_cookie(
                cookie_names_device_id.to_string(),
                refresh_token.device_id,
                cookie_names_device_id.ttl_in_mins(),
            );

            Ok(())
        },
        Err(err) => Err(err),
    }
}

pub async fn get_refresh_token<'err>(
    state: &Arc<AppState>,
    jar: &Cookies,
    ip_addr: String,
    user: User,
) -> error::Result<'err, RefreshToken>
{
    //either
    //1: if user has a device id, db lookup for token and use that if it exists.
    //2: say frog it and keep genning new ones
    //more compelled to use option 1 since gives more control to suspend accounts
    let repo_refresh = &state.refresh_tokens;

    let device_id_cookie_result =
        jar.get_cookie(auth::CookieNames::DEVICE_ID.as_str());

    match device_id_cookie_result
    {
        Ok(cookie_id) => match repo_refresh
            .get_valid_token(&cookie_id, &user.id)
            .await
        {
            Ok(db_refresh_token) => Ok(db_refresh_token),
            Err(_) =>
            {
                let refresh_token =
                    RefreshToken::create_token(user, ip_addr, Some(cookie_id));

                repo_refresh.create_token(refresh_token).await
            },
        },
        Err(_) =>
        {
            let refresh_token = RefreshToken::create_token(user, ip_addr, None);

            repo_refresh.create_token(refresh_token).await
        },
    }
}
