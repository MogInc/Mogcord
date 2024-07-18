use std::sync::Arc;
use axum::{extract::{self, Path, State}, middleware, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::Deserialize;

use crate::{dto::{ChatDTO, ObjectToDTO}, middleware::auth::{self, Ctx}, model::{chat::Chat, misc::{AppState, ServerError, ServerErrorInfo}}};

pub fn routes_chat(state: Arc<AppState>) -> Router
{
    Router::new()
        .route("/chat", post(create_chat_for_authenticated))
        .route("/chat/:chat_id", get(get_chat_for_authenticated))
        .with_state(state)
        .route_layer(middleware::from_fn(auth::mw_require_regular_auth))
        .route_layer(middleware::from_fn(auth::mw_ctx_resolver))
}

async fn get_chat_for_authenticated(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Path(chat_id): Path<String>
) -> impl IntoResponse
{
    let repo_chat = &state.repo_chat;

    let chat = repo_chat
        .get_chat_by_id(&chat_id)
        .await?;

    let ctx_user_id = &ctx.user_id();
    
    match chat.is_user_part_of_chat(ctx_user_id)
    {
        true => Ok(Json(ChatDTO::obj_to_dto(chat))),
        false => Err(ServerError::ChatDoesNotContainThisUser),
    }
}

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
    Server
    {
        name: String,
        owner_id: String,
    },
}
async fn create_chat_for_authenticated(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    extract::Json(payload): extract::Json<CreateChatRequest>
) -> impl IntoResponse
{
    let repo_chat = &state.repo_chat;
    let repo_user = &state.repo_user;

    //Naive solution
    //when AA gets added, check if chat is allowed to be made
    //also handle chat queu so that opposing users dont get auto dragged in it
    //or make it so only chats with friends can be made

    let ctx_user_id = &ctx.user_id();

    let chat = match payload
    {
        CreateChatRequest::Private { owner_ids } => 
        {

            //reason for this check
            //prevention that an end user just overloads the db with a large fetch req
            let req_owner_size = Chat::private_owner_size();
            let actual_owner_size = owner_ids.len();

            if actual_owner_size != actual_owner_size
            {
                return Err(ServerError::OwnerCountInvalid { expected: req_owner_size, found: actual_owner_size } );
            }

            //can move this inside new method
            if !owner_ids.contains(ctx_user_id)
            {
                return Err(ServerError::ChatNotAllowedToBeMade(ServerErrorInfo::UserCreatingIsNotOwner))
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
                return Err(ServerError::ChatNotAllowedToBeMade(ServerErrorInfo::UserCreatingIsNotOwner))
            }

            let owner = repo_user
                .get_user_by_id(&owner_id)
                .await?;

            let users = repo_user
                .get_users_by_id(user_ids)
                .await?;

            Chat::new_group(name, owner, users)?
        },
        CreateChatRequest::Server { name, owner_id} => 
        {
            //can move this inside new method
            if &owner_id != ctx_user_id
            {
                return Err(ServerError::ChatNotAllowedToBeMade(ServerErrorInfo::UserCreatingIsNotOwner))
            }

            let owner = repo_user
                .get_user_by_id(&owner_id)
                .await?;

            Chat::new_server(name, owner, Vec::new())?
        },
    };

    if repo_chat
        .does_chat_exist(&chat)
        .await?
    {
        return Err(ServerError::ChatAlreadyExists);
    }

    match repo_chat.create_chat(chat).await 
    {
        Ok(chat) => Ok(Json(ChatDTO::obj_to_dto(chat))),
        Err(e) => Err(e),
    }
}