use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use tower_cookies::Cookies;

use crate::handlers::logic;
use crate::model::AppState;
use crate::middleware::auth::Ctx;

//can see this as a logout
pub async fn revoke_token(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    jar: Cookies,
) -> impl IntoResponse
{
    logic::auth::authenticated::revoke_token(&state, &ctx, &jar).await
}
