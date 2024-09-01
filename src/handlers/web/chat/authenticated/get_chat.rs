use axum::extract::{Path, Query, State};
use std::sync::Arc;

use crate::handlers::logic;
use crate::middleware::auth::Ctx;
use crate::model::{AppState, Pagination};

pub async fn get_chat<'a>(
    State(state): State<Arc<AppState>>,
    Path(channel_id): Path<String>,
    ctx: Ctx,
    pagination: Option<Query<Pagination>>,
)
{
    let _ = logic::chats::authenticated::get_chat(&state, &ctx, &channel_id).await;

    let pagination = Pagination::new(pagination);

    let _ =
        logic::message::authenticated::get_messages(&state, &channel_id, &ctx, &pagination).await;
}
