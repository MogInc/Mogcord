use std::sync::Arc;
use axum::{extract::{self, Path, State}, middleware, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::Deserialize;

use crate::{dto::ChatDTO, middleware::auth::{self, Ctx}, model::{chat::{Chat, ChatType}, misc::{AppState, ServerError}}};

pub fn routes_chat(state: Arc<AppState>) -> Router
{
    return Router::new()
        .route("/chat", post(create_chat))
        .route("/chat/:chat_id", get(get_chat))
        .with_state(state)
        .route_layer(middleware::from_fn(auth::mw_require_regular_auth))
        .route_layer(middleware::from_fn(auth::mw_ctx_resolver));
}

async fn get_chat(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Path(chat_id): Path<String>
) -> impl IntoResponse
{
    let repo_chat = &state.repo_chat;

    let chat = repo_chat
        .get_chat_by_id(&chat_id)
        .await?;

    let ctx_user_id = ctx.user_id_ref();
    
    match chat.is_user_part_of_chat(ctx_user_id)
    {
        true => Ok(Json(ChatDTO::obj_to_dto(chat))),
        false => Err(ServerError::ChatDoesNotContainThisUser),
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

async fn create_chat(
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
        return Err(ServerError::OwnerCountInvalid);
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