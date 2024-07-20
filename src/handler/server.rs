use std::sync::Arc;
use axum::{extract::{Path, State}, middleware, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::Deserialize;

use crate::model::{error, server, AppState};
use crate::middleware::auth::{self, Ctx};
use crate::dto::{ChatGetResponse, ObjectToDTO, ServerCreateResponse};

pub fn routes(state: Arc<AppState>) -> Router
{
    Router::new()
        .route("/server", post(create_server_for_authenticated))
        .route("/server/:server_id", get(get_server_for_authenticated))
        .route("/server/:server_id", post(join_server_for_authenticated))
        .with_state(state)
        .route_layer(middleware::from_fn(auth::mw_require_regular_auth))
        .route_layer(middleware::from_fn(auth::mw_ctx_resolver))
}

async fn get_server_for_authenticated(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Path(server_id): Path<String>
) -> impl IntoResponse
{
    let repo_chat = &state.chat;

    let chat = repo_chat
        .get_chat_by_id(&server_id)
        .await?;

    let ctx_user_id = &ctx.user_id_ref();
    
    if !chat.is_user_part_of_chat(ctx_user_id)
    {
        return Err(error::Server::ChatDoesNotContainThisUser);
    }

    Ok(Json(ChatGetResponse::obj_to_dto(chat)))
}

#[derive(Deserialize)]
pub struct CreateServerRequest
{
    name: String,
}
async fn create_server_for_authenticated(
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


async fn join_server_for_authenticated(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Path(server_id): Path<String>,
) -> impl IntoResponse
{
    todo!()
}