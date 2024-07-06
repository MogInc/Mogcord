use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::Deserialize;
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::{middleware::{cookies::{self, AuthCookieNames, CookieManager}, jwt}, model::{misc::{AppState, Hashing, ServerError}, token::RefreshToken}};

pub fn routes_auth(state: Arc<AppState>) -> Router
{
    return Router::new()
        .route("/auth/login", post(login))
        .route("/auth/refresh", get(refresh_token))
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
    cookies: Cookies, 
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse
{
    let repo_user = &state.repo_user;
    let repo_refresh = &state.repo_refresh_token;

    let user = repo_user
        .get_user_by_mail(&payload.mail)
        .await?;

    let _ = Hashing::verify_hash(&payload.password, &user.hashed_password).await?;

    //either 
    //1: if user has a device id, db lookup for token and use that if it exists.
    //2: say frog it and keep genning new ones

    let device_id_option = CookieManager::get_cookie(&cookies,AuthCookieNames::DEVICE_ID.into());

    let mut refresh_token: RefreshToken = RefreshToken::create_token();
    let mut create_new_token = true;


    if let Some(device_id_cookie) = device_id_option
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
            .create_token(refresh_token, &user)
            .await?;

        let cookie_device_id = CookieManager::create_cookie(
            AuthCookieNames::DEVICE_ID.into(), 
            refresh_token.device_id, 
            cookies::COOKIE_DEVICE_ID_TTL_MIN
        );

        cookies.add(cookie_device_id);
    }
    
    match jwt::create_token(&user)
    {
        Ok(token) => 
        {
            let cookie_auth = CookieManager::create_cookie(
                AuthCookieNames::AUTH_TOKEN.into(), 
                token, 
                cookies::COOKIE_ACCES_TOKEN_TTL_MIN
            );
            
            let cookie_refresh = CookieManager::create_cookie(
                AuthCookieNames::AUTH_REFRESH.into(), 
                refresh_token.value,
                cookies::COOKIE_REFRESH_TOKEN_TTL_MIN
            );
            
            cookies.add(cookie_auth);
            cookies.add(cookie_refresh);

            return Ok(());
        },
        Err(err) => Err(err),
    }
}

async fn refresh_token(cookies: Cookies)
{

}