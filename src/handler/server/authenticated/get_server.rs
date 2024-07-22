use std::sync::Arc;
use axum::{extract::{Path, State}, response::IntoResponse, Json};

use crate::{dto::ServerGetResponse, model::{error, AppState}};
use crate::middleware::auth::Ctx;
use crate::dto::ObjectToDTO;


pub async fn get_server(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Path(server_id): Path<String>
) -> impl IntoResponse
{
    let repo_server = &state.servers;

    let ctx_user_id = ctx.user_id_ref();


    let server = repo_server
        .get_server_by_id(&server_id)
        .await?;


    if !server.is_user_part_of_server(ctx_user_id)
    {
        return Err(error::Server::ServerDoesNotContainThisUser);
    }

    Ok(Json(ServerGetResponse::obj_to_dto(server)))
}