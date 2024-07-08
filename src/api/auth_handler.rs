use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::{middleware::{auth::{self, jwt::{self, CreateTokenRequest, TokenStatus}}, cookies::{self, AuthCookieNames, Cookie2}}, model::{misc::{AppState, Hashing, ServerError}, token::RefreshToken}};

pub fn routes_auth(state: Arc<AppState>) -> Router
{
    return Router::new()
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh_token))
        .with_state(state);
}

#[derive(Deserialize)]
struct LoginRequest
{
    mail: String,
    password: String,
}

async fn login(
    State(state): State<Arc<AppState>>,
    jar: Cookies, 
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse
{
    let repo_user = &state.repo_user;
    let repo_refresh = &state.repo_refresh_token;

    let device_id_cookie_name = AuthCookieNames::DEVICE_ID;

    let user = repo_user
        .get_user_by_mail(&payload.mail)
        .await?;

    if !user.flag.is_allowed_on_mogcord()
    {
        return Err(ServerError::IncorrectUserPermissions(user.flag.clone()));
    }

    let _ = Hashing::verify_hash(&payload.password, &user.hashed_password).await?;

    //either 
    //1: if user has a device id, db lookup for token and use that if it exists.
    //2: say frog it and keep genning new ones

    let device_id_cookie_option = jar.get_cookie(device_id_cookie_name.as_str());

    let mut refresh_token = RefreshToken::create_token(user);
    let mut create_new_refresh_token = true;


    if let Some(device_id_cookie) = device_id_cookie_option
    {
        match repo_refresh.get_valid_token_by_device_id(&device_id_cookie).await
        {
            Ok(token) => 
            {
                if token.owner.id == refresh_token.owner.id
                {    
                    refresh_token = token;
                    create_new_refresh_token = false;
                }
            },
            _ => (),
        }
    }

    if create_new_refresh_token
    {
        refresh_token = repo_refresh
            .create_token(refresh_token)
            .await?;

        jar.create_cookie(
            device_id_cookie_name.as_str(), 
            refresh_token.device_id, 
            device_id_cookie_name.ttl_in_mins(),
        );
    }
    
    let user = refresh_token.owner;
    let create_token_request = CreateTokenRequest::new(&user.id, &user.flag);
    
    match jwt::create_token(&create_token_request)
    {
        Ok(token) => 
        {
            let acces_token_cookie_name = AuthCookieNames::AUTH_ACCES;
            let refresh_token_cookie_name = AuthCookieNames::AUTH_REFRESH;

            jar.create_cookie(
                acces_token_cookie_name.as_str(), 
                token, 
                acces_token_cookie_name.ttl_in_mins(), 
            );
            
            //refresh token value always gets rewritten
            //not gonna assume its there when trying to login
            jar.create_cookie(
                refresh_token_cookie_name.as_str(),
                refresh_token.value,
                refresh_token_cookie_name.ttl_in_mins(),
            );

            return Ok(());
        },
        Err(err) => Err(err),
    }
}


async fn refresh_token(
    State(state): State<Arc<AppState>>,
    jar: Cookies
) -> impl IntoResponse
{
    let repo_refresh = &state.repo_refresh_token;

    let acces_token_cookie = jar.get_cookie(AuthCookieNames::AUTH_ACCES.into())
        .ok_or(ServerError::AuthCookieNotFound(AuthCookieNames::AUTH_ACCES))?;

    let claims = jwt::extract_token(&acces_token_cookie, TokenStatus::AllowExpired)?;
   
    let refresh_token_cookie = jar.get_cookie(AuthCookieNames::AUTH_REFRESH.into())
        .ok_or(ServerError::AuthCookieNotFound(AuthCookieNames::AUTH_REFRESH))?;

    let device_id_cookie = jar.get_cookie(AuthCookieNames::DEVICE_ID.into())
        .ok_or(ServerError::AuthCookieNotFound(AuthCookieNames::DEVICE_ID))?;

    let refresh_token = repo_refresh
        .get_valid_token_by_device_id(&device_id_cookie)
        .await?;

    if !refresh_token.owner.flag.is_allowed_on_mogcord()
    {
        jar.remove_cookie(AuthCookieNames::AUTH_ACCES.into());
        jar.remove_cookie(AuthCookieNames::AUTH_REFRESH.into());
        return Err(ServerError::IncorrectUserPermissions(refresh_token.owner.flag.clone()));
    }

    if refresh_token.value != refresh_token_cookie
    {
        return Err(ServerError::RefreshTokenDoesNotMatchDeviceId);
    }

    let create_token_request = CreateTokenRequest::new(&claims.sub, &refresh_token.owner.flag);

    match jwt::create_token(&create_token_request)
    {
        Ok(token) => 
        {
            let acces_token_cookie_name = AuthCookieNames::AUTH_ACCES;

            jar.create_cookie(
                acces_token_cookie_name.as_str(), 
                token, 
                acces_token_cookie_name.ttl_in_mins(),
            );
            
            return Ok(());
        },
        Err(err) => Err(err),
    }
}