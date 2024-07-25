use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::dto::{ObjectToDTO, ServerCreateResponse};
use crate::model::channel_parent::server;
use crate::model::AppState;
use crate::middleware::auth::Ctx;


#[derive(Deserialize)]
pub struct CreateServerRequest
{
    name: String,
}
pub async fn create_server(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<CreateServerRequest>
) -> impl IntoResponse
{
    let repo_server = &state.servers;
    let repo_user = &state.users;

    let ctx_user_id = &ctx.user_id();

    let owner = repo_user
        .get_user_by_id(ctx_user_id)
        .await?;

    let server = server::Server::new(payload.name, owner)?;

    match repo_server.create_server(server).await 
    {
        Ok(server) => Ok(Json(ServerCreateResponse::obj_to_dto(server))),
        Err(err) => Err(err),
    }
}