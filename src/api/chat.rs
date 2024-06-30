use std::sync::Arc;
use axum::{extract::{self, Path, Query, State}, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::Deserialize;

use crate::{dto::{ChatDTO, MessageDTO}, model::{chat::{Chat, ChatType}, message::Message, misc::{AppState, Pagination, ServerError}, user::User}};

pub fn routes_chat(state: Arc<AppState>) -> Router
{
    Router::new()
    .route("/chat", post(post_chat))
    .route("/chat/:id", get(get_chat))
    .route("/chat/:id/messages", get(get_messages))
    .route("/chat/:id/message", post(post_message))
    .with_state(state)
}

async fn get_chat(
    State(state): State<Arc<AppState>>,
    Path(chat_id): Path<String>
) -> impl IntoResponse
{
    let repo_chat = &state.repo_chat;

    match repo_chat.get_chat_by_id(&chat_id).await 
    {
        Ok(chat) => Ok(Json(ChatDTO::obj_to_dto(chat))),
        Err(e) => Err(e),
    }
}

#[derive(Deserialize)]
struct CreateChatRequest
{
    name: Option<String>,
    r#type: ChatType,
    owner_ids: Vec<String>,
    user_ids: Option<Vec<String>>,
}

async fn post_chat(
    State(state): State<Arc<AppState>>,
    extract::Json(payload): extract::Json<CreateChatRequest>
) -> impl IntoResponse
{
    let repo_chat = &state.repo_chat;
    let repo_user = &state.repo_user;

    //Naive solution
    //when AA gets added, check if chat is allowed to be made
    //also handle chat queu so that opposing users dont get auto dragged in it

    if !payload.r#type.is_owner_size_allowed(payload.owner_ids.len())
    {
        return Err(ServerError::InvalidOwnerCount);
    }

    let owners = repo_user
        .get_users_by_ids(payload.owner_ids)
        .await
        .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

    let users = match payload.user_ids
    {
        Some(users) => Some(repo_user
            .get_users_by_ids(users)
            .await
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?
        ),
        None => None,
    };

    let chat = Chat::new(
        payload.name,
        payload.r#type, 
        owners,
        users,
    )?;

    if repo_chat
        .does_chat_exist(&chat)
        .await
        .map_err(|err|  ServerError::UnexpectedError(err.to_string()))?
    {
        return Err(ServerError::ChatAlreadyExists);
    }

    match repo_chat.create_chat(chat).await 
    {
        Ok(chat) => Ok(Json(ChatDTO::obj_to_dto(chat))),
        Err(e) => Err(e),
    }
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
    owner_id: String,
}
async fn post_message(
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