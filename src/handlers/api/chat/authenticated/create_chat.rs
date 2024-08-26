use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

use crate::dto::{ChatCreateResponse, ObjectToDTO};
use crate::middleware::auth::Ctx;
use crate::model::channel_parent::chat::Chat;
use crate::model::{channel_parent, error, AppState};
use crate::server_error;

#[derive(Deserialize)]
pub enum CreateChatRequest
{
    Private
    {
        user_id: String
    },
    Group
    {
        name: String, user_ids: Vec<String>
    },
}
pub async fn create_chat(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<CreateChatRequest>,
) -> impl IntoResponse
{
    let repo_chat = &state.chats;
    let repo_user = &state.users;
    let repo_relation = &state.relations;

    let ctx_user_id = &ctx.user_id();

    let chat = match payload
    {
        CreateChatRequest::Private { user_id } =>
        {
            if &user_id == ctx_user_id
            {
                return Err(server_error!(error::Kind::IsSelf, error::OnType::Chat)
                    .add_client(error::Client::CHAT_ADD_WITH_SELF));
            }

            if !repo_relation
                .does_friendship_exist(ctx_user_id, &user_id)
                .await?
            {
                return Err(
                    server_error!(error::Kind::NotFound, error::OnType::RelationFriend)
                        .add_client(error::Client::CHAT_ADD_NON_FRIEND),
                );
            }

            let owners = repo_user
                .get_users_by_id(vec![ctx_user_id.to_string(), user_id])
                .await?;

            let private = channel_parent::chat::Private::new(owners)?;

            Chat::Private(private)
        }
        CreateChatRequest::Group { name, user_ids } =>
        {
            let owner = repo_user.get_user_by_id(ctx_user_id).await?;

            let users = repo_user.get_users_by_id(user_ids).await?;

            let user_ids: Vec<&str> = users.iter().map(|user| &*user.id).collect();

            if !repo_relation
                .does_friendships_exist(ctx_user_id, user_ids)
                .await?
            {
                return Err(
                    server_error!(error::Kind::NotFound, error::OnType::RelationFriend)
                        .add_client(error::Client::CHAT_ADD_NON_FRIEND),
                );
            }

            let group = channel_parent::chat::Group::new(name, owner, users)?;

            Chat::Group(group)
        }
    };

    if repo_chat.does_chat_exist(&chat).await?
    {
        return Err(
            server_error!(error::Kind::AlreadyExists, error::OnType::Chat)
                .add_client(error::Client::CHAT_ALREADY_EXISTS),
        );
    }

    match repo_chat.create_chat(chat).await
    {
        Ok(chat) => Ok(Json(ChatCreateResponse::obj_to_dto(chat))),
        Err(e) => Err(e),
    }
}
