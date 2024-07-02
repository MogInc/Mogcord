use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Json, Router};

use crate::model::misc::AppState;

pub fn routes_auth(state: Arc<AppState>) -> Router
{
    Router::new()
    .route("/auth", get(test))
    .with_state(state)
}

async fn test() -> impl IntoResponse
{
    Json("test")
}