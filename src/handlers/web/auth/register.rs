use askama::Template;
use axum::response::IntoResponse;
use axum_htmx::HxRedirect;

use crate::{handlers::web::HtmxError, middleware::auth::Ctx};

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

    let page = Register
    {
        title: "Register",
        nav_button_value: "Login",
        nav_button_crud_type: "get",
        nav_button_route: "/login",
    };

    Ok((HxRedirect("/register".parse().unwrap()), page).into_response())
}