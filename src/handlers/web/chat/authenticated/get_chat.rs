use askama::Template;
use axum::extract::{Path, Query, State};
use axum::response::Html;
use axum_htmx::HxRequest;
use std::sync::Arc;

use crate::dto::{
    vec_to_dto, vec_to_dto_with_user, ChatGetResponse, MessageGetResponse, ObjectToDTO,
};
use crate::handlers::logic;
use crate::handlers::web::model::HtmxError;
use crate::handlers::web::HeadComponent;
use crate::middleware::auth::Ctx;
use crate::model::{AppState, Pagination};

#[derive(Template)]
#[template(path = "app/chat.html")]
pub struct ChatPage<'a>
{
    head: HeadComponent<'a>,
    chat: ChatGetResponse,
    chats: Vec<ChatGetResponse>,
    messages: Vec<MessageGetResponse>,
}

#[derive(Template)]
#[template(path = "components/chat.html")]
pub struct ChatComponent
{
    chat: ChatGetResponse,
    messages: Vec<MessageGetResponse>,
}

pub async fn get_chat<'a>(
    HxRequest(is_htmx): HxRequest,
    State(state): State<Arc<AppState>>,
    Path(channel_id): Path<String>,
    ctx: Ctx,
    pagination: Option<Query<Pagination>>,
) -> Result<Html<String>, HtmxError>
{
    let chat: ChatGetResponse = logic::chats::authenticated::get_chat(&state, &ctx, &channel_id)
        .await
        .map_err(|err| HtmxError::new(err.client))
        .map(|chat| ObjectToDTO::obj_to_dto_with_user(chat, ctx.user_id_ref()))?;

    let pagination = Pagination::new(pagination);

    let messages =
        logic::message::authenticated::get_messages(&state, &channel_id, &ctx, &pagination)
            .await
            .map_err(|err| HtmxError::new(err.client))
            .map(vec_to_dto)?;

    let html = if is_htmx
    {
        ChatComponent {
            chat,
            messages,
        }
        .render()
        .unwrap()
    }
    else
    {
        let chats = logic::chats::authenticated::get_chats(&state, &ctx).await;
        let ctx_user = ctx.user_id_ref();

        let chats =
            if let Ok(chats) = chats { vec_to_dto_with_user(chats, ctx_user) } else { Vec::new() };

        ChatPage {
            head: HeadComponent::new(chat.name.clone().as_str()),
            chat,
            chats,
            messages,
        }
        .render()
        .unwrap()
    };

    Ok(Html(html))
}
