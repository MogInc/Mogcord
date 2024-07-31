use std::sync::Arc;

use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::State, http::StatusCode, response::{Html, Redirect}, Form};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::{handlers::{logic, web::{server_error_to_display, ErrorComponent}}, model::AppState};

#[derive(Template)]
#[template(path = "login.html")]
pub struct Login
{

}
pub async fn get_login() -> Login
{
    Login{}
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
) -> Result<Redirect, ErrorComponent<'a>> 
{
    let result = logic::auth::login(state, jar, &form.mail, &form.password).await;

    if let Err(err) = result 
    {
        Err(ErrorComponent { message: server_error_to_display(err) })
    } 
    else 
    {
        Ok(Redirect::permanent("/index"))
    }
}