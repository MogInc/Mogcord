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
    let repo_message = &state.message;
    let repo_chat = &state.chat;
    let repo_user = &state.user;

    let ctx_user_id = &ctx.user_id_ref();

    let chat = repo_chat
        .get_chat_by_channel_id(&channel_id)
        .await?;

    if !chat.is_user_part_of_chat(ctx_user_id)
    {
        return Err(error::Server::ChatDoesNotContainThisUser);
    }

    let owner = repo_user
        .get_user_by_id(ctx_user_id)
        .await?;

    let channel = chat.channel();

    let message = Message::new(payload.value, owner, channel);

    match repo_message.create_message(message).await
    {
        Ok(message) => Ok(Json(MessageCreateResponse::obj_to_dto(message))),
        Err(e) => Err(e),
    }
}