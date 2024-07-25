pub mod authenticated;

use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::model::{error, refresh_token::RefreshToken, user, AppState, Hashing};
use crate::middleware::{auth::{self, CreateAccesTokenRequest, TokenStatus}, cookies::Manager};

#[derive(Deserialize)]
pub struct LoginRequest
{
    mail: String,
    password: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    jar: Cookies, 
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse
{
    let repo_user = &state.users;
    let repo_refresh = &state.refresh_tokens;

    let cookie_names_device_id = auth::CookieNames::DEVICE_ID;

    let user = repo_user
        .get_user_by_mail(&payload.mail)
        .await?;

    if !user.flag.is_allowed_on_mogcord()
    {
        return Err(error::Server::IncorrectUserPermissions
            { 
                expected_min_grade: user::Flag::None, 
                found: user.flag.clone()
            }
        );
    }

    Hashing::verify_hash(&payload.password, &user.hashed_password).await?;

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
    let create_token_request = CreateAccesTokenRequest::new(&user.id, &user.flag);
    
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


pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    jar: Cookies
) -> impl IntoResponse
{
    let repo_refresh = &state.refresh_tokens;

    let acces_token_cookie = jar.get_cookie(auth::CookieNames::AUTH_ACCES.as_str())?;

    let claims = auth::extract_acces_token(&acces_token_cookie, &TokenStatus::AllowExpired)?;
   
    let refresh_token_cookie = jar.get_cookie(auth::CookieNames::AUTH_REFRESH.as_str())?;

    let device_id_cookie = jar.get_cookie(auth::CookieNames::DEVICE_ID.as_str())?;

    let refresh_token = repo_refresh
        .get_valid_token_by_device_id(&device_id_cookie)
        .await?;

    if !refresh_token.owner.flag.is_allowed_on_mogcord()
    {
        jar.remove_cookie(auth::CookieNames::AUTH_ACCES.to_string());
        jar.remove_cookie(auth::CookieNames::AUTH_REFRESH.to_string());
        return Err(error::Server::IncorrectUserPermissions
            { 
                expected_min_grade: user::Flag::None, 
                found: refresh_token.owner.flag
            }
        );
    }

    if refresh_token.value != refresh_token_cookie
    {
        return Err(error::Server::RefreshTokenDoesNotMatchDeviceId);
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