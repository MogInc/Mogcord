use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::model::{AppState, error};
use crate::middleware::auth::Ctx;


#[derive(Deserialize)]
pub struct ConfirmFriendRequest
{
    user_id: String,
}
pub async fn confirm_friend(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<ConfirmFriendRequest>,
) -> impl IntoResponse
{
    let repo_relation = &state.relation;

    let ctx_user_id = &ctx.user_id_ref();
    let other_user_id = &payload.user_id;

    if ctx_user_id == other_user_id
    {
        return Err(error::Server::UserYoureAddingCantBeSelf);
    }

    if !repo_relation.does_incoming_friendship_exist(ctx_user_id, other_user_id).await?
    {
        return Err(error::Server::IncomingFriendRequestNotFound);
    }

    if repo_relation.does_friendship_exist(ctx_user_id, other_user_id).await?
    {
        return Err(error::Server::UserIsAlreadyFriend);
    }

    match repo_relation.confirm_user_as_friend(ctx_user_id, other_user_id).await
    {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}

