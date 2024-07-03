use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Json, Router};
use tower_cookies::{Cookie, Cookies};

use crate::{middleware::{cookies::{self, AuthCookieNames}, jwt}, model::{misc::{AppState, ServerError}, user::User}};

pub fn routes_auth(state: Arc<AppState>) -> Router
{
    Router::new()
    .route("/auth", get(test))
    .with_state(state)
}

async fn test(cookies: Cookies) -> impl IntoResponse
{
    let auth_cookie_name: &str = AuthCookieNames::AUTH_TOKEN.into();

    let random_user = User::new(String::from("Snibert"), String::from("Sni@bert.com"));

    match jwt::create_token(&random_user)
    {
        Ok(token) => 
        {
            let cookiebuilder = cookies::create_base_cookie(auth_cookie_name.to_string(), token, jwt::JWT_TTL_MINS);

            cookies.add(cookiebuilder);

            return Ok(());
        },
        Err(err) => Err(err),
    }
}