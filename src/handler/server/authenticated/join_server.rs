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
    todo!()
}