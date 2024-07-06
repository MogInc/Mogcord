use std::sync::Arc;
use axum::{extract::{self, Path, Query, State}, middleware, response::IntoResponse, routing::{get, patch, post}, Json, Router};
use serde::Deserialize;

use crate::{dto::MessageDTO, middleware::Ctx, model::{message::Message, misc::{AppState, Pagination, ServerError}}};
use crate::middleware as mw;

pub fn routes_message(state: Arc<AppState>) -> Router
{
    return Router::new()
        .route("/chat/:chat_id/messages", get(get_messages))
        .route("/chat/:chat_id/message", post(create_message))
        .route("/chat/:chat_id/message/:message_id", patch(update_message))
        .with_state(state)
        .route_layer(middleware::from_fn(mw::mw_require_auth))
        .route_layer(middleware::from_fn(mw::mw_ctx_resolver));
}

async fn get_messages(
    State(state, ): State<Arc<AppState>>,
    Path(chat_id): Path<String>,
    ctx: Ctx,
    pagination: Option<Query<Pagination>>,
) -> impl IntoResponse
{
    let repo_message = &state.repo_message;
    let repo_chat = &state.repo_chat;

    let pagination = Pagination::new(pagination);
    let current_user_id = ctx.user_id_ref();

    let chat = repo_chat
        .get_chat_by_id(&chat_id)
        .await?;

    if !chat.is_user_part_of_chat(&current_user_id)
    {
        return Err(ServerError::ChatDoesNotContainThisUser);
    }

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
}
async fn create_message(
    State(state, ): State<Arc<AppState>>,
    Path(chat_id): Path<String>,
    ctx: Ctx,
    extract::Json(payload): extract::Json<CreateMessageRequest>,
) -> impl IntoResponse
{
    let repo_message = &state.repo_message;
    let repo_chat = &state.repo_chat;
    let repo_user = &state.repo_user;

    let current_user_id = ctx.user_id_ref();

    let chat = repo_chat
        .get_chat_by_id(&chat_id)
        .await?;

    if !chat.is_user_part_of_chat(&current_user_id)
    {
        return Err(ServerError::ChatDoesNotContainThisUser);
    }

    let owner = repo_user
        .get_user_by_id(&current_user_id)
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
}
async fn update_message(
    State(state, ): State<Arc<AppState>>,
    Path((chat_id, message_id)): Path<(String, String)>,
    ctx: Ctx,
    extract::Json(payload): extract::Json<UpdateMessageRequest>,
) -> impl IntoResponse
{
    let repo_message = &state.repo_message;

    let current_user_id = ctx.user_id_ref();
    
    let mut message = repo_message
        .get_message(&message_id)
        .await?;
    
    if !message.is_chat_part_of_message(&chat_id)
    {
        return Err(ServerError::MessageDoesNotContainThisChat);
    }

    if !message.is_user_allowed_to_edit_message(&current_user_id)
    {
        return Err(ServerError::MessageDoesNotContainThisUser);
    }

    message.update_value(payload.value);

    match repo_message.update_message(message).await
    {
        Ok(message) =>  Ok(Json(MessageDTO::obj_to_dto(message))),
        Err(err) => Err(err),
    }
}