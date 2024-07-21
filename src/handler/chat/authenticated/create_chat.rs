use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::model::{chat::Chat, error, AppState};
use crate::middleware::auth::Ctx;
use crate::dto::{ChatCreateResponse, ObjectToDTO};

#[derive(Deserialize)]
pub enum CreateChatRequest
{
    Private
    {
        owner_ids: Vec<String>,
    },
    Group
    {
        name: String,
        owner_id: String,
        user_ids: Vec<String>,
    },
}
pub async fn create_chat(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<CreateChatRequest>
) -> impl IntoResponse
{
    let repo_chat = &state.chat;
    let repo_user = &state.user;

    //Naive solution
    //when AA gets added, check if chat is allowed to be made
    //also handle chat queu so that opposing users dont get auto dragged in it
    //or make it so only chats with friends can be made

    //TODO stop asking owner_id and use ctx, for private ask opposite owner instead of vec

    let ctx_user_id = &ctx.user_id();

    let chat = match payload
    {
        CreateChatRequest::Private { owner_ids } => 
        {

            //reason for this check
            //prevention that an end user just overloads the db with a large fetch req
            let req_owner_size = Chat::private_owner_size();
            let actual_owner_size = owner_ids.len();

            if req_owner_size != actual_owner_size
            {
                return Err(error::Server::OwnerCountInvalid { expected: req_owner_size, found: actual_owner_size } );
            }

            //can move this inside new method
            if !owner_ids.contains(ctx_user_id)
            {
                return Err(error::Server::ChatNotAllowedToBeMade(error::ExtraInfo::UserCreatingIsNotOwner))
            }

            let owners = repo_user
                .get_users_by_id(owner_ids)
                .await?;

            Chat::new_private(owners)?
        },
        CreateChatRequest::Group { name, owner_id, user_ids } => 
        {
            //can move this inside new method
            if &owner_id != ctx_user_id
            {
                return Err(error::Server::ChatNotAllowedToBeMade(error::ExtraInfo::UserCreatingIsNotOwner))
            }

            let owner = repo_user
                .get_user_by_id(&owner_id)
                .await?;

            let users = repo_user
                .get_users_by_id(user_ids)
                .await?;

            Chat::new_group(name, owner, users)?
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