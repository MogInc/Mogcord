use std::sync::Arc;

use askama::Template;
use axum::{extract::State, http::StatusCode, response::{Html, IntoResponse, Redirect}, Form};
use axum_htmx::HxRedirect;
use serde::Deserialize;
use tower_cookies::{cookie::CookieJar, Cookies};

use crate::{handlers::{logic, web::{ErrorComponent}}, model::{error, AppState}};

#[derive(Template)]
#[template(path = "login.html")]
pub struct Login
{

}
pub async fn get_login() -> Result<(), error::Client>
{
    Err(error::Client::PERMISSION_NO_ADMIN)
}

#[derive(Deserialize)]
pub struct LoginRequest
{
    mail: String,
    password: String,
}
pub async fn post_login<'a>(
    State(state): State<Arc<AppState>>,
    jar: Cookies,
    Form(form): Form<LoginRequest>
) -> Result<impl IntoResponse, error::Client>
{
    let result = logic::auth::login(state, jar, &form.mail, &form.password).await;

    if let Err(err) = result 
    {
        Err(err.client)
    } 
    else 
    {
        Ok((HxRedirect("/".parse().unwrap()), "").into_response())
    }
}