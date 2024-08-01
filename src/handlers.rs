pub mod api;
pub mod web;
mod logic;

use std::sync::Arc;
use axum::{http::StatusCode, middleware, response::IntoResponse, routing::Router};
use tower_cookies::CookieManagerLayer;

use crate::{middleware::{auth::mw_ctx_resolver, logging::main_response_mapper}, model::AppState};

pub fn new(state: &Arc<AppState>) -> Router
{
    Router::new()
        .nest("/", web::routes(state.clone()))
        .nest("/api", api::routes(state.clone()))
        .layer(middleware::map_response_with_state(state.logs.clone(), main_response_mapper))
        .layer(middleware::from_fn(mw_ctx_resolver))
        .layer(CookieManagerLayer::new())
        .fallback(page_not_found)
}

async fn page_not_found() -> impl IntoResponse 
{
    (StatusCode::NOT_FOUND, "404 Page Not Found")
}