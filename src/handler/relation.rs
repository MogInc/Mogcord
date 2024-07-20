use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::model::{AppState, error};
use crate::middleware::auth::Ctx;


#[derive(Deserialize)]
pub struct RelationRequest
{
    user_id: String,
}


pub async fn add_friend_auth(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<RelationRequest>,
) -> impl IntoResponse
{
    let repo_relation = &state.relation;
    let repo_user = &state.user;

    let ctx_user_id = &ctx.user_id_ref();
    let other_user_id = &payload.user_id;

    if ctx_user_id == other_user_id
    {
        return Err(error::Server::UserYoureAddingCantBeSelf);
    }

    if !repo_user.does_user_exist_by_id(other_user_id).await?
    {
        return Err(error::Server::UserYoureAddingNotFound);
    }

    if repo_relation.does_blocked_exist(ctx_user_id, other_user_id).await?
    {
        return Err(error::Server::UserYoureAddingIsBlocked);
    }

    if repo_relation.does_blocked_exist(other_user_id, ctx_user_id).await?
    {
        return Err(error::Server::UserYoureAddingHasYouBlocked);
    }

    if repo_relation.does_friendship_exist(ctx_user_id, other_user_id).await?
    {
        return Err(error::Server::UserIsAlreadyFriend);
    }

    match repo_relation.add_user_as_friend(ctx_user_id, other_user_id).await
    {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn confirm_friend_auth(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<RelationRequest>,
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


pub async fn remove_friend_auth(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<RelationRequest>,
) -> impl IntoResponse
{
    let repo_relation = &state.relation;

    let ctx_user_id = &ctx.user_id_ref();
    let other_user_id = &payload.user_id;

    if ctx_user_id == other_user_id
    {
        return Err(error::Server::UserYoureAddingCantBeSelf);
    }

    //no clue if i need more checks as like is_user_a_friend
    //maybe is handy if remove_user_as_friend is expensive and end users are spamming endpoint
    match repo_relation.remove_user_as_friend(ctx_user_id, other_user_id).await
    {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn add_blocked_auth(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<RelationRequest>,
) -> impl IntoResponse
{
    let repo_relation = &state.relation;
    let repo_user = &state.user;

    let ctx_user_id = &ctx.user_id_ref();
    let other_user_id = &payload.user_id;

    if ctx_user_id == other_user_id
    {
        return Err(error::Server::UserYoureAddingCantBeSelf);
    }

    if !repo_user.does_user_exist_by_id(other_user_id).await?
    {
        return Err(error::Server::UserYoureAddingNotFound);
    }

    if repo_relation.does_blocked_exist(ctx_user_id, other_user_id).await?
    {
        return Err(error::Server::UserIsAlreadyBlocked);
    }

    match repo_relation.add_user_as_blocked(ctx_user_id, other_user_id).await
    {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn remove_blocked_auth(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<RelationRequest>,
) -> impl IntoResponse
{
    let repo_relation = &state.relation;

    let ctx_user_id = &ctx.user_id_ref();
    let other_user_id = &payload.user_id;

    if ctx_user_id == other_user_id
    {
        return Err(error::Server::UserYoureAddingCantBeSelf);
    }

    //no clue if i need more checks as like is_user_blocked
    //maybe is handy if remove_user_as_blocked is expensive and end users are spamming endpoint
    match repo_relation.remove_user_as_blocked(ctx_user_id, other_user_id).await
    {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}