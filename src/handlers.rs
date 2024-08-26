mod api;
pub mod logic;
mod web;

use axum::http::StatusCode;
use axum::middleware;
use axum::response::IntoResponse;
use axum::routing::Router;
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;

use crate::middleware::auth::mw_ctx_resolver;
use crate::middleware::logging::main_response_mapper;
use crate::model::AppState;

pub fn new(state: Arc<AppState>) -> Router
{
    Router::new()
        .nest("/", web::routes(state.clone()))
        .nest("/api", api::routes(state.clone()))
        .layer(middleware::map_response_with_state(
            state.logs.clone(),
            main_response_mapper,
        ))
        .layer(middleware::from_fn_with_state(state, mw_ctx_resolver))
        .layer(CookieManagerLayer::new())
        .fallback(page_not_found)
}

async fn page_not_found() -> impl IntoResponse
{
    (StatusCode::NOT_FOUND, "404 Page Not Found")
}
