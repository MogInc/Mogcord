use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::model::channel_parent;
use crate::model::channel_parent::chat::Chat;
use crate::model::{error, AppState};
use crate::middleware::auth::Ctx;
use crate::dto::{ChatCreateResponse, ObjectToDTO};

#[derive(Deserialize)]
pub enum CreateChatRequest
{
    Private
    {
        user_id: String,
    },
    Group
    {
        name: String,
        user_ids: Vec<String>,
    },
}
pub async fn create_chat(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<CreateChatRequest>
) -> impl IntoResponse
{
    let repo_chat = &state.chats;
    let repo_user = &state.users;
    let repo_relation = &state.relations;


    let ctx_user_id = &ctx.user_id();

    //TODO
    //add relation check

    let chat = match payload
    {
        CreateChatRequest::Private { user_id } => 
        {
            if &user_id == ctx_user_id
            {
                return Err(error::Server::ChatNotAllowedToBeMade(error::ExtraInfo::CantHaveChatWithSelf));
            }



            let owners = repo_user
                .get_users_by_id(vec![ctx_user_id.to_string(), user_id])
                .await?;

            let private = channel_parent::chat::Private::new(owners)?;

            Chat::Private(private)
        },
        CreateChatRequest::Group { name, user_ids } => 
        {
            let owner = repo_user
                .get_user_by_id(ctx_user_id)
                .await?;

            let users = repo_user
                .get_users_by_id(user_ids)
                .await?;

            let group = channel_parent::chat::Group::new(name, owner, users)?;

            Chat::Group(group)
        },
    };

    if repo_chat
        .does_chat_exist(&chat)
        .await?
    {
        return Err(error::Server::ChatAlreadyExists);
    }

    match repo_chat.create_chat(chat).await 
    {
        Ok(chat) => Ok(Json(ChatCreateResponse::obj_to_dto(chat))),
        Err(e) => Err(e),
    }
}