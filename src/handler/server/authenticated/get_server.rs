use std::sync::Arc;
use axum::{extract::{Path, State}, response::IntoResponse, Json};

use crate::model::{error, AppState};
use crate::middleware::auth::Ctx;
use crate::dto::ObjectToDTO;


pub async fn get_server(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Path(server_id): Path<String>
) -> impl IntoResponse
{
    todo!()
}