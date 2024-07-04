use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::Deserialize;
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::{middleware::{cookies::{self, AuthCookieNames}, jwt, refresh_token_creator, Ctx}, model::misc::AppState};

pub fn routes_auth(state: Arc<AppState>) -> Router
{
    Router::new()
    .route("/auth/login", post(login))
    .route("/auth/refresh", get(refresh_token))
    .with_state(state)
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

    let acces_token_name: &str = AuthCookieNames::AUTH_TOKEN.into();
    let refresh_token_name: &str = AuthCookieNames::AUTH_REFRESH.into();
    let device_id_name: &str = AuthCookieNames::DEVICE_ID.into();

    match jwt::create_token(&user)
    {
        Ok(token) => 
        {
            //TODO: add that to DB
            let cookie_auth = cookies::create_cookie(acces_token_name, token, cookies::COOKIE_ACCES_TOKEN_TTL_MIN);
            let cookie_refresh = cookies::create_cookie(refresh_token_name, refresh_token_creator::create_refresh_token(), cookies::COOKIE_REFRESH_TOKEN_TTL_MIN);
            let cookie_device_id = cookies::create_cookie(device_id_name, Uuid::now_v7().to_string(), cookies::COOKIE_DEVICE_ID_TTL_MIN);
            
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