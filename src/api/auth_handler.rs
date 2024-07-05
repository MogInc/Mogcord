use std::{sync::Arc, time::Instant};

use axum::{extract::State, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::Deserialize;
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::{middleware::{cookies::{self, AuthCookieNames}, jwt, RefreshTokenCreater}, model::misc::{AppState, Hashing, ServerError}};

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

    match jwt::create_token(&user)
    {
        Ok(token) => 
        {
            //TODO: add that to DB
            let cookie_auth = cookies::create_cookie(
                AuthCookieNames::AUTH_TOKEN.into(), 
                token, 
                cookies::COOKIE_ACCES_TOKEN_TTL_MIN
            );
            let cookie_refresh = cookies::create_cookie(
                AuthCookieNames::AUTH_REFRESH.into(), 
                RefreshTokenCreater::create_refresh_token(), 
                cookies::COOKIE_REFRESH_TOKEN_TTL_MIN
            );
            let cookie_device_id = cookies::create_cookie(
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