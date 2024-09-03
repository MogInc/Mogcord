use askama::Template;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use std::sync::Arc;

use crate::dto::{vec_to_dto, MessageGetResponse};
use crate::handlers::logic;
use crate::handlers::web::model::HtmxError;
use crate::middleware::auth::Ctx;
use crate::model::{AppState, Pagination};

#[derive(Template)]
#[template(path = "app/chat.html")]
pub struct ChatPage
{
    messages: Vec<MessageGetResponse>,
}

pub async fn get_chat<'a>(
    State(state): State<Arc<AppState>>,
    Path(channel_id): Path<String>,
    ctx: Ctx,
    pagination: Option<Query<Pagination>>,
) -> Result<impl IntoResponse, HtmxError>
{
    let _ = logic::chats::authenticated::get_chat(&state, &ctx, &channel_id).await;

    let pagination = Pagination::new(pagination);

    let messages =
        logic::message::authenticated::get_messages(&state, &channel_id, &ctx, &pagination)
            .await
            .map_err(|err| HtmxError::new(err.client))
            .map(vec_to_dto)?;

    Ok(ChatPage {
        messages,
    })
}
