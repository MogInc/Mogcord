use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::IntoResponse, Form};
use axum_htmx::HxRedirect;
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::{handlers::{logic, web::HtmxError}, middleware::auth::Ctx, model::AppState};

#[derive(Template)]
#[template(path = "login.html")]
pub struct Login<'a>
{
    title: &'a str,
    nav_button_value: &'a str,
    nav_button_crud: &'a str,
    nav_button_route: &'a str,
}
pub async fn get_login<'a>(ctx_option: Option<Ctx>) -> Result<Login<'a>, HtmxError>
{
    if ctx_option.is_some()
    {
        return Err(HtmxError::new(crate::model::error::Client::USER_ALREADY_LOGGED_IN));
    }

    Ok(
        Login
        {
            title: "Login",
            nav_button_value: "Register",
            nav_button_crud: "get",
            nav_button_route: "/register",
        }
    )
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
    ctx_option: Option<Ctx>,
    Form(form): Form<LoginRequest>
) -> Result<impl IntoResponse, HtmxError>
{
    if ctx_option.is_some()
    {
        return Err(HtmxError::new(crate::model::error::Client::USER_ALREADY_LOGGED_IN));
    }

    let login_result = logic::auth::login(state, jar, &form.mail, &form.password).await;

    if let Err(err) = login_result 
    {
        Err(HtmxError::new_form_error(err.client))
    } 
    else 
    {
        Ok((HxRedirect("/".parse().unwrap()), "").into_response())
    }
}