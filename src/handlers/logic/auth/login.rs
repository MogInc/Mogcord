use std::sync::Arc;
use tower_cookies::Cookies;


use crate::middleware::cookies::Manager;
use crate::model::{error, refresh_token::RefreshToken, AppState, Hashing};
use crate::middleware::auth::{self, CreateAccesTokenRequest};
use crate::server_error;


pub async fn login<'err>(
    state: Arc<AppState>,
    jar: Cookies,
    email: &str,
    password: &str,
) -> Result<(), error::Server<'err>>
{
    let repo_user = &state.users;
    let repo_refresh = &state.refresh_tokens;

    let cookie_names_device_id = auth::CookieNames::DEVICE_ID;

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

    let device_id_cookie_result = jar.get_cookie(cookie_names_device_id.as_str());

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

        jar.create_cookie(
            cookie_names_device_id.to_string(), 
            refresh_token.device_id, 
            cookie_names_device_id.ttl_in_mins(),
        );
    }
    
    let user = refresh_token.owner;
    let create_token_request = CreateAccesTokenRequest::new(&user.id, user.flag.is_mogcord_admin_or_owner());
    
    match auth::create_acces_token(&create_token_request)
    {
        Ok(acces_token) => 
        {
            let cookie_names_acces_token = auth::CookieNames::AUTH_ACCES;
            let cookie_names_refresh_token = auth::CookieNames::AUTH_REFRESH;

            jar.create_cookie(
                cookie_names_acces_token.to_string(), 
                acces_token, 
                cookie_names_acces_token.ttl_in_mins(), 
            );
            
            //refresh token value always gets rewritten
            //not gonna assume its there when trying to login
            jar.create_cookie(
                cookie_names_refresh_token.to_string(),
                refresh_token.value,
                cookie_names_refresh_token.ttl_in_mins(),
            );

            Ok(())
        },
        Err(err) => Err(err),
    }
}