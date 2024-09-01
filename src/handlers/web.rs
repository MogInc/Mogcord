mod auth;
mod chat;
mod message;
mod middleware;
mod misc;
mod model;

use crate::middleware::auth::mw_require_authentication;
use crate::model::{error, AppState};
use askama::Template;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::routing::{get, post};
use axum::Router;
use middleware::mw_require_htmx_authentication;
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

pub fn routes(state: Arc<AppState>) -> Router
{
    let routes_with_regular_middleware = Router::new()
        //auth
        .route("/logout", post(auth::authenticated::logout))
        .route_layer(axum::middleware::from_fn(mw_require_authentication))
        .with_state(state.clone());

    let routes_with_htmx_regular_middleware = Router::new()
        //channels
        .route("/channels", get(chat::authenticated::get_chats))
        .route("/channels/:channel_id", get(message::authenticated::get_messages))
        .route_layer(axum::middleware::from_fn(mw_require_htmx_authentication))
        .with_state(state.clone());

    let routes_without_middleware = Router::new()
        //auth
        .route("/login", get(auth::get_login).post(auth::post_login))
        .route("/register", get(auth::get_register).post(auth::post_register))
        //index
        .route("/", get(misc::index))
        //public files
        .nest_service("/pub", ServeDir::new("public/assets"))
        .nest_service("/robots.txt", ServeFile::new("public/robots.txt"))
        .with_state(state);

    Router::new()
        .merge(routes_with_regular_middleware)
        .merge(routes_with_htmx_regular_middleware)
        .merge(routes_without_middleware)
}

pub struct NavbarComponent<'a>
{
    button_value: &'a str,
    button_crud_type: &'a str,
    button_route: &'a str,
    links: Vec<NavbarLink<'a>>,
}

pub struct NavbarLink<'a>
{
    value: &'a str,
    redirect: &'a str,
}

#[derive(Template)]
#[template(path = "components/alerts/error.html")]
pub struct AlertErrorComponent<'a>
{
    message: &'a str,
}

impl IntoResponse for model::HtmxError
{
    fn into_response(self) -> axum::response::Response
    {
        #[allow(clippy::match_same_arms)]
        match self.client
        {
            error::Client::PERMISSION_NO_ADMIN
            | error::Client::NOT_ALLOWED_PLATFORM
            | error::Client::PERMISSION_NO_AUTH => Redirect::temporary("/").into_response(),
            error::Client::USER_ALREADY_LOGGED_IN => Redirect::temporary("/").into_response(),
            error::Client::SERVICE_ERROR =>
                (StatusCode::INTERNAL_SERVER_ERROR, error::Client::SERVICE_ERROR.translate_error())
                    .into_response(),
            rest if self.display == model::PotentialErrorDisplay::Alert => (
                StatusCode::BAD_REQUEST,
                AlertErrorComponent {
                    message: rest.translate_error(),
                },
            )
                .into_response(),
            rest => (StatusCode::BAD_REQUEST, rest.translate_error()).into_response(),
        }
    }
}
