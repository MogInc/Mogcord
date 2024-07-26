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

    let server = repo_server
        .get_server_by_id(&server_id)
        .await?;

    let ctx_user_id = ctx.user_id_ref();
    
    if !server.is_user_part_of_server(ctx_user_id)
    {
        return Err(error::Server::new(
            error::Kind::NotAllowed,
            error::OnType::Server,
            file!(),
            line!())
        );
    }

    let server = server.filter_channels(ctx_user_id);

    Ok(Json(ServerGetResponse::obj_to_dto(server)))
}