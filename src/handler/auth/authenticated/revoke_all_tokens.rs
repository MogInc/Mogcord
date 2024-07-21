use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};

use crate::model::AppState;
use crate::middleware::auth::Ctx;

pub async fn revoke_all_tokens(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
) -> impl IntoResponse
{
    let repo_refresh = &state.refresh_token;
    
    let ctx_user_id = &ctx.user_id_ref();

    match repo_refresh.revoke_all_tokens(ctx_user_id).await
    {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}