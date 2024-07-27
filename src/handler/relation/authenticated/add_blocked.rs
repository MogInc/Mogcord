use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::model::{AppState, error};
use crate::middleware::auth::Ctx;


#[derive(Deserialize)]
pub struct AddBlockedRequest
{
    user_id: String,
}

pub async fn add_blocked(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<AddBlockedRequest>,
) -> impl IntoResponse
{
    let repo_relation = &state.relations;
    let repo_user = &state.users;

    let ctx_user_id = &ctx.user_id_ref();
    let other_user_id = &payload.user_id;

    if ctx_user_id == other_user_id
    {
        return Err(error::Server::new(
            error::Kind::IsSelf,
            error::OnType::RelationBlocked,
            file!(),
            line!())
            .add_client(error::Client::RELATION_SELF_TRY_BLOCK_SELF)
        );
    }

    if !repo_user.does_user_exist_by_id(other_user_id).await?
    {
        return Err(error::Server::new(
            error::Kind::NotFound,
            error::OnType::User,
            file!(),
            line!())
            .add_debug_info("user to be blocked", other_user_id.to_string())
        );
    }

    if repo_relation.does_blocked_exist(ctx_user_id, other_user_id).await?
    {
        return Err(error::Server::new(
            error::Kind::InValid,
            error::OnType::RelationBlocked,
            file!(),
            line!())
            .add_client(error::Client::RELATION_USER_ALREADY_BLOCKED)
        );
    }

    match repo_relation.add_user_as_blocked(ctx_user_id, other_user_id).await
    {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}