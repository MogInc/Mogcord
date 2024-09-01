use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use crate::dto::{ChatGetResponse, ObjectToDTO};
use crate::handlers::logic;
use crate::middleware::auth::Ctx;
use crate::model::{error, AppState};

pub async fn get_chat(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Path(chat_id): Path<String>,
) -> impl IntoResponse
{
    let chat = logic::chats::authenticated::get_chat(&state, &ctx, &chat_id).await?;

    Ok::<Json<ChatGetResponse>, error::Server>(Json(ChatGetResponse::obj_to_dto(chat)))
}
