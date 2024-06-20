use std::sync::Arc;
use axum::{extract::{Path, State}, response::IntoResponse, routing::{get, post}, Json, Router};

use crate::{db::mongoldb::MongolDB, model::chat::ChatRepository};

pub fn routes_user(state: Arc<MongolDB>) -> Router
{
    Router::new()
    .route("/chat/:id", get(get_chat))
    .route("/chat", post(post_chat))
    .with_state(state)
}

async fn get_chat(
    State(db): State<Arc<dyn ChatRepository>>,
    Path(uuid): Path<String>) 
    -> impl IntoResponse
{
    match db.get_chat_by_id(&uuid).await 
    {
        Ok(chat) => Ok(Json(chat)),
        Err(e) => Err(e),
    }
}

async fn post_chat()
{

}