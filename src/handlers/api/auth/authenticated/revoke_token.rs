use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use tower_cookies::Cookies;

use crate::handlers::logic;
use crate::middleware::auth::Ctx;
use crate::model::AppState;

//can see this as a logout
pub async fn revoke_token(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    jar: Cookies,
) -> impl IntoResponse
{
    logic::auth::authenticated::revoke_token(&state, &ctx, &jar).await
}
