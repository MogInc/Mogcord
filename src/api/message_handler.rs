use std::sync::Arc;
use axum::{extract::{self, Path, Query, State}, response::IntoResponse, routing::{get, patch, post}, Json, Router};
use serde::Deserialize;
use tokio::sync::oneshot::error;

use crate::{dto::MessageDTO, model::{chat::Chat, message::Message, misc::{AppState, Pagination, ServerError}, user::User}};

pub fn routes_message(state: Arc<AppState>) -> Router
{
    Router::new()
    .route("/chat/:chat_id/messages", get(get_messages))
    .route("/chat/:chat_id/message", post(create_message))
    .route("/chat/:chat_id/message/:message_id", patch(update_message))
    .with_state(state)
}

async fn get_messages(
    State(state, ): State<Arc<AppState>>,
    Path(chat_id): Path<String>,
    pagination: Option<Query<Pagination>>,
) -> impl IntoResponse
{
    let repo_message = &state.repo_message;
    let pagination = Pagination::new(pagination);

    match repo_message.get_messages(&chat_id, pagination).await
    {
        Ok(messages) => Ok(Json(MessageDTO::vec_to_dto(messages))),
        Err(e) => Err(e),
    }
}

#[derive(Deserialize)]
struct CreateMessageRequest
{
    value: String,
    //TODO: replace with cookie or any form of other AA
    owner_id: String,
}
async fn create_message(
    State(state, ): State<Arc<AppState>>,
    Path(chat_id): Path<String>,
    extract::Json(payload): extract::Json<CreateMessageRequest>,
) -> impl IntoResponse
{
    let repo_message = &state.repo_message;
    let repo_chat = &state.repo_chat;
    let repo_user = &state.repo_user;

    let chat: Chat = repo_chat
        .get_chat_by_id(&chat_id)
        .await?;

    if !chat.is_user_part_of_chat(&payload.owner_id)
    {
        return Err(ServerError::UserNotPartOfThisChat);
    }

    let owner: User = repo_user
        .get_user_by_id(&payload.owner_id)
        .await?;

    let message = Message::new(payload.value, owner, chat);

    match repo_message.create_message(message).await
    {
        Ok(message) => Ok(Json(MessageDTO::obj_to_dto(message))),
        Err(e) => Err(e),
    }
}

#[derive(Deserialize)]
struct UpdateMessageRequest
{
    value: String,
    //TODO: replace with cookie or any form of other AA
    owner_id: String,
}
async fn update_message(
    State(state, ): State<Arc<AppState>>,
    Path((chat_id, message_id)): Path<(String, String)>,
    extract::Json(payload): extract::Json<UpdateMessageRequest>,
) -> impl IntoResponse
{
    let repo_message = &state.repo_message;

    //TODO
    //Retrieve message
    //retrieve chat -> is in message
    //validate 
    //change message
    //return changed message

    let message = repo_message
        .get_message(&message_id)
        .await?;

    if !message.is_chat_part_of_message(&chat_id)
    {
        return Err(ServerError::ChatNotPartThisMessage);
    }

    if !message.is_user_allowed_to_edit_message(&payload.owner_id)
    {
        return Err(ServerError::ChatNotPartThisMessage);
    }

    return Ok(Json(MessageDTO::obj_to_dto(message)));
}