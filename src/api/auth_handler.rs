use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::{middleware::{cookies::{self, AuthCookieNames, Cookie2}, jwt::{self, CreateTokenRequest, TokenStatus}}, model::{misc::{AppState, Hashing, ServerError}, token::RefreshToken}};

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

    let device_id_cookie_name : &str = AuthCookieNames::DEVICE_ID.into();

    let user = repo_user
        .get_user_by_mail(&payload.mail)
        .await?;

    let _ = user
        .flag
        .is_allowed_on_platform()?;

    let _ = Hashing::verify_hash(&payload.password, &user.hashed_password).await?;

    //either 
    //1: if user has a device id, db lookup for token and use that if it exists.
    //2: say frog it and keep genning new ones

    let device_id_cookie_option = jar.get_cookie(device_id_cookie_name);

    let mut refresh_token = RefreshToken::create_token(user);
    let mut create_new_token = true;


    if let Some(device_id_cookie) = device_id_cookie_option
    {
        match repo_refresh.get_valid_token_by_device_id(&device_id_cookie).await
        {
            Ok(token) => 
            {
                if token.owner.id == refresh_token.owner.id
                {    
                    refresh_token = token;
                    create_new_token = false;
                }
            },
            _ => (),
        }
    }

    if create_new_token
    {
        refresh_token = repo_refresh
            .create_token(refresh_token)
            .await?;

        jar.create_cookie(
            device_id_cookie_name, 
            refresh_token.device_id, 
            cookies::COOKIE_DEVICE_ID_TTL_MIN
        );
    }
    
    let user = refresh_token.owner;
    let create_token_request = CreateTokenRequest::new(&user.id, &user.flag);
    
    match jwt::create_token(&create_token_request)
    {
        Ok(token) => 
        {
            jar.create_cookie(
                AuthCookieNames::AUTH_ACCES.into(), 
                token, 
                cookies::COOKIE_ACCES_TOKEN_TTL_MIN
            );
            
            jar.create_cookie(
                AuthCookieNames::AUTH_REFRESH.into(), 
                refresh_token.value,
                cookies::COOKIE_REFRESH_TOKEN_TTL_MIN
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

    if let Err(err) = refresh_token.owner.flag.is_allowed_on_platform()
    {
        jar.remove_cookie(AuthCookieNames::AUTH_ACCES.into());
        jar.remove_cookie(AuthCookieNames::AUTH_REFRESH.into());
        return Err(err);
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
            jar.create_cookie(
                AuthCookieNames::AUTH_ACCES.into(), 
                token, 
                cookies::COOKIE_ACCES_TOKEN_TTL_MIN
            );
            
            return Ok(());
        },
        Err(err) => Err(err),
    }
}