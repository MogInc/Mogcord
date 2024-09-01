use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use std::sync::Arc;

use crate::handlers::logic;
use crate::middleware::auth::Ctx;
use crate::model::{AppState, Pagination};

pub async fn get_messages(
    State(state): State<Arc<AppState>>,
    Path(channel_id): Path<String>,
    ctx: Ctx,
    pagination: Option<Query<Pagination>>,
) -> impl IntoResponse
{
    let pagination = Pagination::new(pagination);
    let _ =
        logic::message::authenticated::get_messages(&state, &channel_id, &ctx, &pagination).await;
}
