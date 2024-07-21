use std::sync::Arc;
use axum::{extract::{self, Path, State}, response::IntoResponse, Json};
use serde::Deserialize;

use crate::model::{error, AppState};
use crate::middleware::auth::Ctx;
use crate::dto::{MessageCreateResponse, ObjectToDTO};

#[derive(Deserialize)]
pub struct UpdateMessageRequest
{
    value: String,
}
pub async fn update_message(
    State(state, ): State<Arc<AppState>>,
    Path((channel_id, message_id)): Path<(String, String)>,
    ctx: Ctx,
    extract::Json(payload): extract::Json<UpdateMessageRequest>,
) -> impl IntoResponse
{
    let repo_message = &state.message;

    let ctx_user_id = ctx.user_id_ref();
    
    let mut message = repo_message
        .get_message(&message_id)
        .await?;
    
    if !message.is_chat_part_of_message(&channel_id)
    {
        return Err(error::Server::MessageDoesNotContainThisChat);
    }

    if !message.is_user_allowed_to_edit_message(ctx_user_id)
    {
        return Err(error::Server::MessageDoesNotContainThisUser);
    }

    message.update_value(payload.value);

    match repo_message.update_message(message).await
    {
        Ok(message) =>  Ok(Json(MessageCreateResponse::obj_to_dto(message))),
        Err(err) => Err(err),
    }
}