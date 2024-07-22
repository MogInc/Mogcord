use std::sync::Arc;
use axum::{extract::{Path, Query, State}, response::IntoResponse, Json};

use crate::model::{error, message::Message, AppState, Pagination};
use crate::middleware::auth::Ctx;
use crate::dto::{vec_to_dto, MessageGetResponse};


pub async fn get_messages(
    State(state, ): State<Arc<AppState>>,
    Path(channel_id): Path<String>,
    ctx: Ctx,
    pagination: Option<Query<Pagination>>,
) -> impl IntoResponse
{
    let repo_message = &state.messages;
    let repo_chat = &state.chats;
    let repo_channel = &state.channels;

    let pagination = Pagination::new(pagination);
    let current_user_id = &ctx.user_id_ref();

    // let channel = repo_channel
    //     .get_channel(&channel_id)
    //     .await?;

    todo!()
}
