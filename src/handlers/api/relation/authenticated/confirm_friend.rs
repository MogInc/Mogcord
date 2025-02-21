use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use crate::middleware::auth::Ctx;
use crate::model::{
    error,
    AppState,
};
use crate::server_error;

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
    let repo_relation = &state.relations;

    let ctx_user_id = &ctx.user_id_ref();
    let other_user_id = &payload.user_id;

    if ctx_user_id == other_user_id
    {
        return Err(server_error!(
            error::Kind::IsSelf,
            error::OnType::RelationFriend
        )
        .add_client(error::Client::RELATION_SELF_TRY_FRIEND_SELF));
    }

    if !repo_relation
        .does_incoming_friendship_exist(ctx_user_id, other_user_id)
        .await?
    {
        return Err(server_error!(
            error::Kind::NotFound,
            error::OnType::RelationFriend
        )
        .add_client(error::Client::RELATION_NO_INCOMING_FRIEND));
    }

    if repo_relation
        .does_friendship_exist(ctx_user_id, other_user_id)
        .await?
    {
        return Err(server_error!(
            error::Kind::AlreadyExists,
            error::OnType::RelationFriend
        )
        .add_client(error::Client::RELATION_USER_ALREADY_FRIEND));
    }

    match repo_relation
        .confirm_user_as_friend(ctx_user_id, other_user_id)
        .await
    {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}
