use std::sync::Arc;
use axum::{extract::State, response::IntoResponse};
use tower_cookies::Cookies;

use crate::model::AppState;
use crate::handlers;

pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    jar: Cookies
) -> impl IntoResponse
{
    handlers::logic::auth::refresh_token(&state, &jar).await
}