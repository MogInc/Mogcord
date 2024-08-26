use std::net::SocketAddr;
use std::sync::Arc;

use askama::Template;
use axum::extract::{ConnectInfo, State};
use axum::response::IntoResponse;
use axum::Form;
use axum_htmx::HxRedirect;
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::handlers::logic;
use crate::handlers::web::HtmxError;
use crate::middleware::auth::Ctx;
use crate::model::AppState;

#[derive(Template)]
#[template(path = "register.html")]
pub struct Register<'a>
{
    title: &'a str,
    nav_button_value: &'a str,
    nav_button_crud_type: &'a str,
    nav_button_route: &'a str,
}
pub async fn get_register(ctx_option: Option<Ctx>) -> Result<impl IntoResponse, HtmxError>
{
    if ctx_option.is_some()
    {
        return Err(HtmxError::new(crate::model::error::Client::USER_ALREADY_LOGGED_IN));
    }

    let page = Register {
        title: "Register",
        nav_button_value: "Login",
        nav_button_crud_type: "get",
        nav_button_route: "/login",
    };

    Ok((HxRedirect("/register".parse().unwrap()), page).into_response())
}

#[derive(Deserialize)]
pub struct RegisterRequest
{
    email: String,
    username: String,
    password: String,
    confirm_password: String,
}
pub async fn post_register(
    State(state): State<Arc<AppState>>,
    jar: Cookies,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ctx_option: Option<Ctx>,
    Form(form): Form<RegisterRequest>,
) -> Result<impl IntoResponse, HtmxError>
{
    if ctx_option.is_some()
    {
        return Err(HtmxError::new(crate::model::error::Client::USER_ALREADY_LOGGED_IN));
    }

    if form.password != form.confirm_password
    {
        return Err(HtmxError::new(crate::model::error::Client::PASSWORD_CONFIRM_NOT_MATCH));
    }

    let create_request =
        logic::user::CreateUserRequest::new(form.username, form.email, form.password);

    let user = logic::user::create_user(&state, &create_request)
        .await
        .map_err(|err| HtmxError::new_form_error(err.client))?;

    //schedule some task to see if ban evader

    let refresh_token =
        logic::auth::cookies::get_refresh_token(&state, &jar, addr.to_string(), user)
            .await
            .map_err(|err| HtmxError::new_form_error(err.client))?;

    logic::auth::cookies::create_auth_cookies(&jar, refresh_token)
        .map_err(|err| HtmxError::new_form_error(err.client))?;

    Ok((HxRedirect("/".parse().unwrap()), "").into_response())
}
