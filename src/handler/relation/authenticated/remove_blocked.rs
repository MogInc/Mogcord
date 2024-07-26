use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::model::{AppState, error};
use crate::middleware::auth::Ctx;


#[derive(Deserialize)]
pub struct RemoveBlockedRequest
{
    user_id: String,
}
pub async fn remove_blocked(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<RemoveBlockedRequest>,
) -> impl IntoResponse
{
    let repo_relation = &state.relations;

    let ctx_user_id = &ctx.user_id_ref();
    let other_user_id = &payload.user_id;

    if ctx_user_id == other_user_id
    {
        return Err(error::Server::new(
            error::Kind::IsSelf,
            error::OnType::RelationBlocked,
            file!(),
            line!())
            .add_client(error::Client::TRY_ADD_SELF_BLOCKED)
        );
    }

    //no clue if i need more checks as like is_user_blocked
    //maybe is handy if remove_user_as_blocked is expensive and end users are spamming endpoint
    match repo_relation.remove_user_as_blocked(ctx_user_id, other_user_id).await
    {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}