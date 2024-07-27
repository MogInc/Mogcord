use std::sync::Arc;
use axum::{extract::{Path, State}, response::IntoResponse, Json};
use serde::Deserialize;

use crate::model::{error, AppState};
use crate::middleware::auth::Ctx;

#[derive(Deserialize)]
pub struct AddUsersRequest
{
    user_ids: Vec<String>,
}

pub async fn add_users_to_chat(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Path(chat_id): Path<String>,
    Json(payload): Json<AddUsersRequest>,
) -> impl IntoResponse
{
    let repo_chat = &state.chats;
    let repo_relation = &state.relations;
    let repo_user = &state.users;
    
    let ctx_user_id = ctx.user_id_ref();

    let mut chat = repo_chat
        .get_chat_by_id(&chat_id)
        .await?;

    if !chat.is_group()
    {   
        return Err(error::Server::new(
            error::Kind::CantGainUsers,
            error::OnType::ChatPrivate,
            file!(),
            line!())
            .add_client(error::Client::CHAT_CANT_GAIN_USERS)
        );
    }

    if !chat.is_owner(ctx_user_id)
    {
        return Err(error::Server::new(
            error::Kind::IncorrectPermissions,
            error::OnType::Chat,
            file!(),
            line!())
            .add_client(error::Client::CHAT_EDIT_NOT_OWNER)
        );
    }

    let user_ids: Vec<&str> = payload
        .user_ids
        .iter()
        .map(AsRef::as_ref)
        .collect();
    
    if repo_relation.does_friendships_exist(ctx_user_id, user_ids).await?
    {
        return Err(error::Server::new(
            error::Kind::NotFound,
            error::OnType::RelationFriend,
            file!(),
            line!())
            .add_client(error::Client::CHAT_ADD_NON_FRIEND)
        );
    }

    let users = repo_user
        .get_users_by_id(payload.user_ids)
        .await?;

    chat.add_users(users)?;

    
    match repo_chat.update_chat(chat).await
    {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}