use std::sync::Arc;

use axum::extract::State;

use crate::handlers::logic;
use crate::middleware::auth::Ctx;
use crate::model::AppState;

pub async fn get_chats(State(state): State<Arc<AppState>>, ctx: Ctx)
{
    let chats = logic::chats::authenticated::get_chats(&state, &ctx).await;

    if let Ok(chat) = chats
    {
        println!("{chat:?}");
    }
}
