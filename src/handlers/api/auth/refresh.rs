use axum::extract::State;
use axum::response::IntoResponse;
use std::sync::Arc;
use tower_cookies::Cookies;

use crate::handlers;
use crate::middleware::auth::Ctx;
use crate::model::AppState;

pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    ctx_option: Option<Ctx>,
    jar: Cookies,
) -> impl IntoResponse
{
    if ctx_option.is_some()
    {
        return Ok(());
    }

    handlers::logic::auth::refresh_token(&state, &jar).await
}
