use std::sync::Arc;
use axum::{extract::{self, Path, State}, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::Deserialize;

use crate::{db::mongoldb::MongolDB, model::{chat::{Chat, ChatError, ChatRepository, ChatType}, user::{User, UserRepository}}};

pub fn routes_chat(state: Arc<MongolDB>) -> Router
{
    Router::new()
    .route("/chat/:id", get(get_chat))
    .route("/chat", post(post_chat))
    .with_state(state)
}

async fn get_chat(
    State(repo): State<Arc<dyn ChatRepository>>,
    Path(uuid): Path<String>) 
    -> impl IntoResponse
{
    match repo.get_chat_by_id(&uuid).await 
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
    State(repo_chat): State<Arc<dyn ChatRepository>>,
    State(repo_user): State<Arc<dyn UserRepository>>,
    extract::Json(payload): extract::Json<CreateChatRequest>)
 -> impl IntoResponse
{
    if Chat::is_owner_size_allowed(payload.r#type, payload.owners.len())
    {
        return Err(ChatError::InvalidOwnerCount);
    }

    

    let chat: Chat = Chat::new(
        payload.name,
        payload.r#type, 
        users.clone(),
        None,
    )?;

    match repo_chat.create_chat(chat).await 
    {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e),
    }
}