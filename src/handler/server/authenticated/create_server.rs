use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::model::{server, AppState};
use crate::middleware::auth::Ctx;
use crate::dto::{ObjectToDTO, ServerCreateResponse};


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
    let repo_server = &state.server;
    let repo_user = &state.user;

    let ctx_user_id = &ctx.user_id();

    let owner = repo_user
        .get_user_by_id(ctx_user_id)
        .await?;

    let server = server::Server::new(payload.name, owner)?;

    match repo_server.create_server(server).await 
    {
        Ok(server) => Ok(Json(ServerCreateResponse::obj_to_dto(server))),
        Err(e) => Err(e),
    }
}