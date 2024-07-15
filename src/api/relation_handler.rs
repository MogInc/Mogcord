use std::sync::Arc;

use axum::{extract::State, middleware, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;

use crate::{middleware::auth::{self, Ctx}, model::misc::{AppState, ServerError}};

pub fn routes_relation(state: Arc<AppState>) -> Router
{
    Router::new()
        //.route("/friends", todo!())
        .route("/friends", post(add_friend_for_authenticated))
        .route("/blocked", post(add_blocked_for_authenticated))
        //.route("/blocked", todo!())
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

    let ctx_user_id = ctx.user_id_ref();
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

async fn add_blocked_for_authenticated(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<RelationRequest>,
) -> impl IntoResponse
{
    let repo_relation = &state.repo_relation;
    let repo_user = &state.repo_user;

    let ctx_user_id = ctx.user_id_ref();
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