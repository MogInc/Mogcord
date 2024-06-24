use std::sync::Arc;
use axum::{extract::{self, Path, State}, response::IntoResponse, routing::{get, post}, Json, Router};
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
    members: Option<Vec<String>>,
}

async fn post_chat(
    State(state): State<Arc<AppState>>,
    extract::Json(payload): extract::Json<CreateChatRequest>)
 -> impl IntoResponse
{
    let repo_chat = &state.repo_chat;
    let repo_user = &state.repo_user;

    if !Chat::is_owner_size_allowed(&payload.r#type, payload.owners.len())
    {
        return Err(ServerError::InvalidOwnerCount);
    }

    //TODO: make sure a chat is unique

    let owners = repo_user
        .get_users_by_id(payload.owners)
        .await
        .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

    let members = match payload.members
    {
        Some(members) => Some(repo_user
            .get_users_by_id(members)
            .await
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?
        ),
        None => None,
    };

    let chat = Chat::new(
        payload.name,
        payload.r#type, 
        owners.clone(),
        members,
    )?;

    match repo_chat.create_chat(chat).await 
    {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e),
    }
}