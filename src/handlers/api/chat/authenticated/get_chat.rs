use std::sync::Arc;
use axum::{extract::{Path, State}, response::IntoResponse, Json};

use crate::{dto::ChatGetResponse, model::{error, AppState}, server_error};
use crate::middleware::auth::Ctx;
use crate::dto::ObjectToDTO;

pub async fn get_chat(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Path(chat_id): Path<String>
) -> impl IntoResponse
{
    let repo_chat = &state.chats;

    let chat = repo_chat
        .get_chat_by_id(&chat_id)
        .await?;

    let ctx_user_id = ctx.user_id_ref();
    
    if !chat.is_user_part_of_chat(ctx_user_id)
    {
        return Err(server_error!(error::Kind::NotPartOf, error::OnType::Chat)
            .add_client(error::Client::CHAT_CTX_NOT_PART_OF_CHAT)
        );
    }

    Ok(Json(ChatGetResponse::obj_to_dto(chat)))
}