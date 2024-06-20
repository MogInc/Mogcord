use std::sync::Arc;
use axum::{extract::{self, Path, State}, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::Deserialize;

use crate::{db::mongoldb::MongolDB, model::{chat::{Chat, ChatRepository}, user::User}};

pub fn routes_chat(state: Arc<MongolDB>) -> Router
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


    // match db.get_chat_by_id(&uuid).await 
    // {
    //     Ok(chat) => Ok(Json(chat)),
    //     Err(e) => Err(e),
    // }
}

#[derive(Deserialize)]
struct CreateChatRequest
{

}

async fn post_chat(
    State(db): State<Arc<dyn ChatRepository>>,
    extract::Json(payload): extract::Json<CreateChatRequest>)
 -> impl IntoResponse
{
    let mut users: Vec<crate::model::user::User> = Vec::new();
    users.push(User::new(String::from("Ted"), String::from("Ted@shit.com")));
    let chat = Chat::new(Some(String::from("W in da chat")), crate::model::chat::ChatType::Group, users.clone(),Some(users.clone()), None);

    match db.create_chat(chat).await 
    {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e),
    }
}