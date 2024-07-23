use std::sync::Arc;
use axum::{extract::{Path, State}, response::IntoResponse, Json};

use crate::model::{error, AppState};
use crate::middleware::auth::Ctx;
use crate::dto::ObjectToDTO;


pub async fn get_server_channel(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Path((server_id, channel_id)): Path<(String, String)>
) -> impl IntoResponse
{
    todo!()
}