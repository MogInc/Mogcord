use std::sync::Arc;

use askama::Template;
use axum::extract::State;

use crate::dto::{vec_to_dto_with_user, ChatGetResponse};
use crate::handlers::logic;
use crate::middleware::auth::Ctx;
use crate::model::AppState;

#[derive(Template)]
#[template(path = "app/index.html")]
pub struct Index<'a>
{
    title: &'a str,
    chats: Vec<ChatGetResponse>,
}

pub async fn get_chats<'a>(State(state): State<Arc<AppState>>, ctx: Ctx) -> Index<'a>
{
    let chats = logic::chats::authenticated::get_chats(&state, &ctx).await;
    let ctx_user = ctx.user_id_ref();

    let chats =
        if let Ok(chats) = chats { vec_to_dto_with_user(chats, ctx_user) } else { Vec::new() };

    Index {
        title: "App",
        chats,
    }
}
