use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::model::{channel_parent::ChannelParent, error, AppState};
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

    //Naive solution
    //when AA gets added, check if chat is allowed to be made
    //also handle chat queu so that opposing users dont get auto dragged in it
    //or make it so only chats with friends can be made

    //TODO stop asking owner_id and use ctx, for private ask opposite owner instead of vec

    let ctx_user_id = &ctx.user_id();

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

            ChannelParent::new_private(owners)?
        },
        CreateChatRequest::Group { name, user_ids } => 
        {
            let owner = repo_user
                .get_user_by_id(ctx_user_id)
                .await?;

            let users = repo_user
                .get_users_by_id(user_ids)
                .await?;

            ChannelParent::new_group(name, owner, users)?
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