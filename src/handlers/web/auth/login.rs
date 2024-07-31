use std::sync::Arc;

use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::State, response::Html, Form};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::{handlers::logic, model::AppState};

#[derive(Template)]
#[template(path = "login.html")]
pub struct Login
{
    mail: Option<String>,
    password: Option<String>,
    error: Option<String>,
}
pub async fn get_login() -> Login
{
    Login
    {
        mail: None,
        password: None,
        error: None,
    }
}

#[derive(Deserialize)]
pub struct LoginRequest
{
    mail: String,
    password: String,
}
pub async fn post_login(
    State(state): State<Arc<AppState>>,
    jar: Cookies,
    Form(form): Form<LoginRequest>
) -> impl IntoResponse
{
    let result = logic::auth::login(state, jar, form.mail, form.password).await;
}