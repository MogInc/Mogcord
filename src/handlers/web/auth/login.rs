use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::IntoResponse, Form};
use axum_htmx::HxRedirect;
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::{handlers::logic, model::{error, AppState}};

#[derive(Template)]
#[template(path = "login.html")]
pub struct Login
{

}
pub async fn get_login(jar: Cookies) -> Result<Login, error::Client>
{
    super::is_logged_in(&jar)?;

    Ok(Login{})
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
) -> Result<impl IntoResponse, error::Client>
{
    super::is_logged_in(&jar)?;

    let login_result = logic::auth::login(state, jar, &form.mail, &form.password).await;

    if let Err(err) = login_result 
    {
        Err(err.client)
    } 
    else 
    {
        Ok((HxRedirect("/".parse().unwrap()), "").into_response())
    }
}