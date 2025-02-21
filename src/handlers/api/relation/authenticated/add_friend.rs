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
pub struct AddFriendRequest
{
    user_id: String,
}
pub async fn add_friend(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<AddFriendRequest>,
) -> impl IntoResponse
{
    let repo_relation = &state.relations;
    let repo_user = &state.users;

    let ctx_user_id = &ctx.user_id_ref();
    let other_user_id = &payload.user_id;

    if ctx_user_id == other_user_id
    {
        return Err(server_error!(
            error::Kind::IsSelf,
            error::OnType::RelationFriend
        ));
    }

    if !repo_user.does_user_exist_by_id(other_user_id).await?
    {
        return Err(server_error!(
            error::Kind::NotFound,
            error::OnType::User
        )
        .add_debug_info(
            "user to be added",
            other_user_id.to_string(),
        ));
    }

    if repo_relation
        .does_blocked_exist(ctx_user_id, other_user_id)
        .await?
    {
        return Err(server_error!(
            error::Kind::InValid,
            error::OnType::RelationBlocked
        )
        .add_client(error::Client::RELATION_USER_BLOCKED));
    }

    if repo_relation
        .does_blocked_exist(other_user_id, ctx_user_id)
        .await?
    {
        return Err(server_error!(
            error::Kind::NotAllowed,
            error::OnType::Relation
        )
        .add_client(error::Client::RELATION_USER_BLOCKED_YOU));
    }

    if repo_relation
        .does_outgoing_friendship_exist(ctx_user_id, other_user_id)
        .await?
    {
        return Err(server_error!(
            error::Kind::AlreadyExists,
            error::OnType::RelationFriend
        )
        .add_client(error::Client::RELATION_USER_ALREADY_FRIEND));
    }

    match repo_relation
        .add_user_as_friend(ctx_user_id, other_user_id)
        .await
    {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}
