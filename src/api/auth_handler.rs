use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::{middleware::{cookies::{self, AuthCookieNames}, jwt, Ctx}, model::misc::AppState};

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

    let auth_cookie_name: &str = AuthCookieNames::AUTH_TOKEN.into();

    match jwt::create_token(&user)
    {
        Ok(token) => 
        {
            let cookie_auth = cookies::create_cookie(auth_cookie_name.to_string(), token, cookies::JWT_COOKIE_TTL_MINS);

            cookies.add(cookie_auth);

            return Ok(());
        },
        Err(err) => Err(err),
    }
}

async fn refresh_token(cookies: Cookies, ctx: Ctx)
{
    
}