use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use crate::dto::{vec_to_dto, MessageGetResponse};
use crate::middleware::auth::Ctx;
use crate::model::channel::Parent;
use crate::model::message::Message;
use crate::model::{error, AppState, Pagination};
use crate::server_error;

pub async fn get_messages(
    State(state): State<Arc<AppState>>,
    Path(channel_id): Path<String>,
    ctx: Ctx,
    pagination: Option<Query<Pagination>>,
) -> impl IntoResponse
{
    let repo_message = &state.messages;
    let repo_parent = &state.channel_parents;

    let pagination = Pagination::new(pagination);
    let current_user_id = ctx.user_id_ref();

    let chat = repo_parent.get_channel_parent(&channel_id).await?;

    if !chat.is_user_part_of_channel_parent(current_user_id)
    {
        return Err(server_error!(error::Kind::NotPartOf, error::OnType::ChannelParent)
            .add_client(error::Client::SERVER_CTX_NOT_PART_OF_SERVER));
    }

    if !chat.can_read(current_user_id, Some(&channel_id))?
    {
        return Err(server_error!(error::Kind::NotPartOf, error::OnType::ChannelParent)
            .add_client(error::Client::SERVER_CTX_NOT_PART_OF_SERVER));
    }

    match repo_message
        .get_valid_messages(&channel_id, pagination)
        .await
    {
        Ok(messages) => Ok(Json(vec_to_dto::<Message, MessageGetResponse>(messages))),
        Err(e) => Err(e),
    }
}
