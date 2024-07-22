use std::sync::Arc;
use axum::{extract::{self, Path, State}, response::IntoResponse, Json};
use serde::Deserialize;

use crate::model::{error, message::Message, AppState};
use crate::middleware::auth::Ctx;
use crate::dto::{MessageCreateResponse, ObjectToDTO};

#[derive(Deserialize)]
pub struct CreateMessageRequest
{
    value: String,
}
pub async fn create_message(
    State(state, ): State<Arc<AppState>>,
    Path(channel_id): Path<String>,
    ctx: Ctx,
    extract::Json(payload): extract::Json<CreateMessageRequest>,
) -> impl IntoResponse
{
    let repo_message = &state.messages;
    let repo_chat = &state.chats;
    let repo_user = &state.users;
    let repo_channel = &state.channels;

    let ctx_user_id = &ctx.user_id_ref();

    // let channel = repo_channel
    //     .get_channel(&channel_id)
    //     .await?;

    todo!()
}