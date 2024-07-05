use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::Deserialize;
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::{middleware::{cookies::{self, AuthCookieNames, CookieManager}, jwt}, model::{misc::{AppState, Hashing}, token::RefreshToken}};

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

    let user = repo_user
        .get_user_by_mail(&payload.mail)
        .await?;

    let _ = Hashing::verify_hash(&payload.password, &user.hashed_password).await?;

    //either 
    //if user has a device id, token up if exists and use that.
    //say frog it and keep genning new ones

    match jwt::create_token(&user)
    {
        Ok(token) => 
        {
            //TODO: add that to DB
            let cookie_auth = CookieManager::create_cookie(
                AuthCookieNames::AUTH_TOKEN.into(), 
                token, 
                cookies::COOKIE_ACCES_TOKEN_TTL_MIN
            );
            let cookie_refresh = CookieManager::create_cookie(
                AuthCookieNames::AUTH_REFRESH.into(), 
                RefreshToken::create_token().value, 
                cookies::COOKIE_REFRESH_TOKEN_TTL_MIN
            );
            let cookie_device_id = CookieManager::create_cookie(
                AuthCookieNames::DEVICE_ID.into(), 
                Uuid::now_v7().to_string(), 
                cookies::COOKIE_DEVICE_ID_TTL_MIN
            );
            
            cookies.add(cookie_auth);
            cookies.add(cookie_refresh);
            cookies.add(cookie_device_id);

            return Ok(());
        },
        Err(err) => Err(err),
    }
}

async fn refresh_token(cookies: Cookies)
{

}