use std::sync::Arc;
use axum::{extract::{self, Path, Query, State}, response::IntoResponse, Json};
use serde::Deserialize;

use crate::model::{error, message::Message, AppState, Pagination};
use crate::middleware::auth::Ctx;
use crate::dto::{vec_to_dto, MessageCreateResponse, MessageGetResponse, ObjectToDTO};


pub async fn get_messages_auth(
    State(state, ): State<Arc<AppState>>,
    Path(chat_info_id): Path<String>,
    ctx: Ctx,
    pagination: Option<Query<Pagination>>,
) -> impl IntoResponse
{
    let repo_message = &state.message;
    let repo_chat = &state.chat;

    let pagination = Pagination::new(pagination);
    let current_user_id = &ctx.user_id_ref();

    let chat = repo_chat
        .get_chat_by_chat_info_id(&chat_info_id)
        .await?;

    if !chat.is_user_part_of_chat(current_user_id)
    {
        return Err(error::Server::ChatDoesNotContainThisUser);
    }

    match repo_message.get_valid_messages(&chat_info_id, pagination).await
    {
        Ok(messages) => Ok(Json(vec_to_dto::<Message, MessageGetResponse>(messages))),
        Err(e) => Err(e),
    }
}

#[derive(Deserialize)]
pub struct CreateMessageRequest
{
    value: String,
}
pub async fn create_message_auth(
    State(state, ): State<Arc<AppState>>,
    Path(chat_info_id): Path<String>,
    ctx: Ctx,
    extract::Json(payload): extract::Json<CreateMessageRequest>,
) -> impl IntoResponse
{
    let repo_message = &state.message;
    let repo_chat = &state.chat;
    let repo_user = &state.user;

    let ctx_user_id = &ctx.user_id_ref();

    let chat = repo_chat
        .get_chat_by_chat_info_id(&chat_info_id)
        .await?;

    if !chat.is_user_part_of_chat(ctx_user_id)
    {
        return Err(error::Server::ChatDoesNotContainThisUser);
    }

    let owner = repo_user
        .get_user_by_id(ctx_user_id)
        .await?;

    let chat_info = chat.chat_info();

    let message = Message::new(payload.value, owner, chat_info);

    match repo_message.create_message(message).await
    {
        Ok(message) => Ok(Json(MessageCreateResponse::obj_to_dto(message))),
        Err(e) => Err(e),
    }
}

#[derive(Deserialize)]
pub struct UpdateMessageRequest
{
    value: String,
}
pub async fn update_message_auth(
    State(state, ): State<Arc<AppState>>,
    Path((chat_info_id, message_id)): Path<(String, String)>,
    ctx: Ctx,
    extract::Json(payload): extract::Json<UpdateMessageRequest>,
) -> impl IntoResponse
{
    let repo_message = &state.message;

    let ctx_user_id = ctx.user_id_ref();
    
    let mut message = repo_message
        .get_message(&message_id)
        .await?;
    
    if !message.is_chat_part_of_message(&chat_info_id)
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