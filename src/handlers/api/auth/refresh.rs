use std::sync::Arc;
use axum::{extract::State, response::IntoResponse};
use tower_cookies::Cookies;

use crate::middleware::auth::Ctx;
use crate::model::AppState;
use crate::handlers;

pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    ctx_option: Option<Ctx>,
    jar: Cookies
) -> impl IntoResponse
{
    if ctx_option.is_some()
    {
        return Ok(());
    }

    handlers::logic::auth::refresh_token(&state, &jar).await
}