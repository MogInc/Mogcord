use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::{middleware::{cookies::{self, AuthCookieNames, CookieManager}, jwt::{self, CreateTokenRequest, TokenStatus}}, model::{misc::{AppState, Hashing, ServerError}, token::RefreshToken}};

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

    //TODO: add user ban checks

    let user = repo_user
        .get_user_by_mail(&payload.mail)
        .await?;

    let _ = Hashing::verify_hash(&payload.password, &user.hashed_password).await?;

    //either 
    //1: if user has a device id, db lookup for token and use that if it exists.
    //2: say frog it and keep genning new ones

    let device_id_cookie_option = CookieManager::get_cookie(&jar,AuthCookieNames::DEVICE_ID.into());

    let mut refresh_token: RefreshToken = RefreshToken::create_token(user.clone());
    let mut create_new_token = true;


    if let Some(device_id_cookie) = device_id_cookie_option
    {
        match repo_refresh.get_token_by_device_id(&device_id_cookie).await
        {
            Ok(token) => 
            {
                refresh_token = token;
                create_new_token = false;
            },
            _ => (),
        }
    }

    if create_new_token
    {
        refresh_token = repo_refresh
            .create_token(refresh_token)
            .await?;

        let cookie_device_id = CookieManager::create_cookie(
            AuthCookieNames::DEVICE_ID.into(), 
            refresh_token.device_id, 
            cookies::COOKIE_DEVICE_ID_TTL_MIN
        );

        CookieManager::set_cookie(&jar, cookie_device_id);
    }

    let create_token_request = CreateTokenRequest::new(user.id, user.user_flag);
    
    match jwt::create_token(&create_token_request)
    {
        Ok(token) => 
        {
            let cookie_auth = CookieManager::create_cookie(
                AuthCookieNames::AUTH_ACCES.into(), 
                token, 
                cookies::COOKIE_ACCES_TOKEN_TTL_MIN
            );
            
            let cookie_refresh = CookieManager::create_cookie(
                AuthCookieNames::AUTH_REFRESH.into(), 
                refresh_token.value,
                cookies::COOKIE_REFRESH_TOKEN_TTL_MIN
            );

            CookieManager::set_cookie(&jar, cookie_auth);
            CookieManager::set_cookie(&jar, cookie_refresh);

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

    let acces_token_cookie = CookieManager::get_cookie(&jar, AuthCookieNames::AUTH_ACCES.into())
        .ok_or(ServerError::AuthCookieNotFound(AuthCookieNames::AUTH_ACCES))?;

    let claims = jwt::extract_token(&acces_token_cookie, TokenStatus::AllowExpired)?;
   
    let refresh_token_cookie = CookieManager::get_cookie(&jar, AuthCookieNames::AUTH_REFRESH.into())
        .ok_or(ServerError::AuthCookieNotFound(AuthCookieNames::AUTH_REFRESH))?;

    let device_id_cookie = CookieManager::get_cookie(&jar, AuthCookieNames::DEVICE_ID.into())
        .ok_or(ServerError::AuthCookieNotFound(AuthCookieNames::DEVICE_ID))?;

    let refresh_token = repo_refresh
        .get_token_by_device_id(&device_id_cookie)
        .await?;

    if refresh_token.owner.user_flag.is_yeeted()
    {
        CookieManager::remove_cookie(&jar, AuthCookieNames::AUTH_ACCES.into());
        CookieManager::remove_cookie(&jar, AuthCookieNames::AUTH_REFRESH.into());
        return Err(ServerError::IncorrectPermissions);
    }

    if refresh_token.value != refresh_token_cookie
    {
        return Err(ServerError::RefreshTokenDoesNotMatchDeviceId);
    }

    let create_token_request = CreateTokenRequest::new(claims.sub, refresh_token.owner.user_flag);

    match jwt::create_token(&create_token_request)
    {
        Ok(token) => 
        {
            let cookie_auth = CookieManager::create_cookie(
                AuthCookieNames::AUTH_ACCES.into(), 
                token, 
                cookies::COOKIE_ACCES_TOKEN_TTL_MIN
            );
            
            CookieManager::set_cookie(&jar, cookie_auth);

            return Ok(());
        },
        Err(err) => Err(err),
    }
}