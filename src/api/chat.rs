use std::sync::Arc;
use axum::{extract::{self, Path, State}, response::IntoResponse, routing::{get, post}, Json, Router};
use futures_util::{FutureExt, TryFutureExt};
use serde::Deserialize;

use crate::model::{appstate::AppState, chat::{Chat, ChatType}, error::ServerError};

pub fn routes_chat(state: Arc<AppState>) -> Router
{
    Router::new()
    .route("/chat/:id", get(get_chat))
    .route("/chat", post(post_chat))
    .with_state(state)
}

async fn get_chat(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<String>) 
    -> impl IntoResponse
{
    let repo_chat = &state.repo_chat;

    match repo_chat.get_chat_by_id(&uuid).await 
    {
        Ok(chat) => Ok(Json(chat)),
        Err(e) => Err(e),
    }
}

#[derive(Deserialize)]
struct CreateChatRequest
{
    name: Option<String>,
    r#type: ChatType,
    owners: Vec<String>,
    users: Option<Vec<String>>,
}

async fn post_chat(
    State(state): State<Arc<AppState>>,
    extract::Json(payload): extract::Json<CreateChatRequest>)
 -> impl IntoResponse
{
    let repo_chat = &state.repo_chat;
    let repo_user = &state.repo_user;

    //Naive solution
    //when AA gets added, check if chat is allowed to be made
    //also handle chat queu so that opposing users dont get auto dragged in it

    if !payload.r#type.is_owner_size_allowed(payload.owners.len())
    {
        return Err(ServerError::InvalidOwnerCount);
    }

    //TODO: make sure a chat is unique

    let owners = repo_user
        .get_users_by_id(payload.owners)
        .await
        .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

    let users = match payload.users
    {
        Some(users) => Some(repo_user
            .get_users_by_id(users)
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
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e),
    }
}