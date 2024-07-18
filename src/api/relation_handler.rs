use std::sync::Arc;

use axum::{extract::State, middleware, response::IntoResponse, routing::{delete, post}, Json, Router};
use serde::Deserialize;

use crate::{middleware::auth::{self, Ctx}, model::misc::{AppState, ServerError}};

pub fn routes_relation(state: Arc<AppState>) -> Router
{
    Router::new()
        .route("/friends", post(add_friend_for_authenticated))
        .route("/friends/confirm", post(confirm_friend_for_authenticated))
        .route("/friends", delete(remove_friend_for_authenticated))
        .route("/blocked", post(add_blocked_for_authenticated))
        .route("/blocked", delete(remove_blocked_for_authenticated))
        .with_state(state)
        .route_layer(middleware::from_fn(auth::mw_require_regular_auth))
        .route_layer(middleware::from_fn(auth::mw_ctx_resolver))
}

#[derive(Deserialize)]
struct RelationRequest
{
    user_id: String,
}


async fn add_friend_for_authenticated(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<RelationRequest>,
) -> impl IntoResponse
{
    let repo_relation = &state.repo_relation;
    let repo_user = &state.repo_user;

    let ctx_user_id = &ctx.user_id();
    let other_user_id = &payload.user_id;

    if ctx_user_id == other_user_id
    {
        return Err(ServerError::UserYoureAddingCantBeSelf);
    }

    if !repo_user.does_user_exist_by_id(&other_user_id).await?
    {
        return Err(ServerError::UserYoureAddingNotFound);
    }

    if repo_relation.does_blocked_exist(&ctx_user_id, &other_user_id).await?
    {
        return Err(ServerError::UserYoureAddingIsBlocked);
    }

    if repo_relation.does_blocked_exist(&other_user_id, &ctx_user_id).await?
    {
        return Err(ServerError::UserYoureAddingHasYouBlocked);
    }

    if repo_relation.does_friendship_exist(&ctx_user_id, &other_user_id).await?
    {
        return Err(ServerError::UserIsAlreadyFriend);
    }

    match repo_relation.add_user_as_friend(&ctx_user_id, &other_user_id).await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

async fn confirm_friend_for_authenticated(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<RelationRequest>,
) -> impl IntoResponse
{
    let repo_relation = &state.repo_relation;

    let ctx_user_id = &ctx.user_id();
    let other_user_id = &payload.user_id;

    if ctx_user_id == other_user_id
    {
        return Err(ServerError::UserYoureAddingCantBeSelf);
    }

    if !repo_relation.does_incoming_friendship_exist(&ctx_user_id, &other_user_id).await?
    {
        return Err(ServerError::IncomingFriendRequestNotFound);
    }

    if repo_relation.does_friendship_exist(&ctx_user_id, &other_user_id).await?
    {
        return Err(ServerError::UserIsAlreadyFriend);
    }

    match repo_relation.confirm_user_as_friend(&ctx_user_id, &other_user_id).await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}


async fn remove_friend_for_authenticated(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<RelationRequest>,
) -> impl IntoResponse
{
    let repo_relation = &state.repo_relation;

    let ctx_user_id = &ctx.user_id();
    let other_user_id = &payload.user_id;

    if ctx_user_id == other_user_id
    {
        return Err(ServerError::UserYoureAddingCantBeSelf);
    }

    //no clue if i need more checks as like is_user_a_friend
    //maybe is handy if remove_user_as_friend is expensive and end users are spamming endpoint
    match repo_relation.remove_user_as_friend(&ctx_user_id, &other_user_id).await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

async fn add_blocked_for_authenticated(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<RelationRequest>,
) -> impl IntoResponse
{
    let repo_relation = &state.repo_relation;
    let repo_user = &state.repo_user;

    let ctx_user_id = &ctx.user_id();
    let other_user_id = &payload.user_id;

    if ctx_user_id == other_user_id
    {
        return Err(ServerError::UserYoureAddingCantBeSelf);
    }

    if !repo_user.does_user_exist_by_id(&other_user_id).await?
    {
        return Err(ServerError::UserYoureAddingNotFound);
    }

    if repo_relation.does_blocked_exist(&ctx_user_id, &other_user_id).await?
    {
        return Err(ServerError::UserIsAlreadyBlocked);
    }

    match repo_relation.add_user_as_blocked(&ctx_user_id, &other_user_id).await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

async fn remove_blocked_for_authenticated(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<RelationRequest>,
) -> impl IntoResponse
{
    let repo_relation = &state.repo_relation;

    let ctx_user_id = &ctx.user_id();
    let other_user_id = &payload.user_id;

    if ctx_user_id == other_user_id
    {
        return Err(ServerError::UserYoureAddingCantBeSelf);
    }

    //no clue if i need more checks as like is_user_blocked
    //maybe is handy if remove_user_as_blocked is expensive and end users are spamming endpoint
    match repo_relation.remove_user_as_blocked(&ctx_user_id, &other_user_id).await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}