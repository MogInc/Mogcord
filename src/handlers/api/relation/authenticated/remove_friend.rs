use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use crate::middleware::auth::Ctx;
use crate::model::{error, AppState};
use crate::server_error;

#[derive(Deserialize)]
pub struct RemoveFriendRequest
{
    user_id: String,
}
pub async fn remove_friend(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<RemoveFriendRequest>,
) -> impl IntoResponse
{
    let repo_relation = &state.relations;

    let ctx_user_id = &ctx.user_id_ref();
    let other_user_id = &payload.user_id;

    if ctx_user_id == other_user_id
    {
        return Err(
            server_error!(error::Kind::IsSelf, error::OnType::RelationFriend)
                .add_client(error::Client::RELATION_SELF_TRY_UNFRIEND_SELF),
        );
    }

    //no clue if i need more checks as like is_user_a_friend
    //maybe is handy if remove_user_as_friend is expensive and end users are
    // spamming endpoint
    match repo_relation
        .remove_user_as_friend(ctx_user_id, other_user_id)
        .await
    {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}
