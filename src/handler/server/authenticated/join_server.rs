use std::sync::Arc;
use axum::{extract::{Path, State}, response::IntoResponse};

use crate::model::{error, AppState};
use crate::middleware::auth::Ctx;

pub async fn join_server(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Path(server_id): Path<String>,
) -> impl IntoResponse
{
    let repo_user = &state.users;
    let repo_server = &state.servers;
    let repo_relation = &state.relations;

    let ctx_user_id = ctx.user_id_ref();

    let mut server = repo_server
        .get_server_by_id(&server_id)
        .await?;

    if repo_relation.does_blocked_exist(&server.owner.id, ctx_user_id).await?
    {
        return Err(error::Server::ServerOwnerHasYouBlocked);
    }

    let user = repo_user
        .get_user_by_id(ctx_user_id)
        .await?;

    server.add_user(user)?;

    match repo_server.add_user_to_server(&server_id, ctx_user_id).await
    {
        Ok(()) => Ok(()),
        Err(_) => Err(error::Server::FailedToAddUserToServer),
    }
}