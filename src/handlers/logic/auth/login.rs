use std::sync::Arc;
use tower_cookies::Cookies;


use crate::handlers::logic;
use crate::middleware::cookies::Manager;
use crate::model::{error, refresh_token::RefreshToken, AppState, Hashing};
use crate::middleware::auth;
use crate::server_error;


pub async fn login<'err>(
    state: Arc<AppState>,
    jar: Cookies,
    email: &str,
    password: &str,
) -> error::Result<'err, ()>
{
    let repo_user = &state.users;
    let repo_refresh = &state.refresh_tokens;

    let user = repo_user
        .get_user_by_mail(email)
        .await.map_err(|err| 
            server_error!(err).add_client(error::Client::INVALID_PARAMS)
        )?;

    if !user.flag.is_allowed_on_mogcord()
    {
        return Err(server_error!(error::Kind::IncorrectPermissions, error::OnType::User)
            .add_client(error::Client::NOT_ALLOWED_PLATFORM)
            .add_debug_info("user flag", user.flag.to_string())
        );
    }

    Hashing::verify_hash(password, &user.hashed_password).await.map_err(|err| 
        server_error!(err).add_client(error::Client::INVALID_PARAMS)
    )?;

    //either 
    //1: if user has a device id, db lookup for token and use that if it exists.
    //2: say frog it and keep genning new ones
    let device_id_cookie_result = jar.get_cookie(auth::CookieNames::DEVICE_ID.as_str());

    let mut refresh_token = RefreshToken::create_token(user);
    let mut create_new_refresh_token = true;


    if let Ok(device_id_cookie) = device_id_cookie_result
    {
        if let Ok(token) = repo_refresh.get_valid_token_by_device_id(&device_id_cookie).await
        {
            if token.owner.id == refresh_token.owner.id
            {    
                refresh_token = token;
                create_new_refresh_token = false;
            }
        }
    }

    if create_new_refresh_token
    {
        refresh_token = repo_refresh
            .create_token(refresh_token)
            .await?;
    }
    
    logic::auth::create_auth_cookies(&jar, refresh_token)
}