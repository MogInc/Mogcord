use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

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
    todo!()
}