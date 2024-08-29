use std::sync::Arc;

use crate::middleware::auth::Ctx;
use crate::model::channel_parent::chat::Chat;
use crate::model::{error, AppState};

pub async fn get_chats<'err>(state: &Arc<AppState>, ctx: &Ctx) -> error::Result<'err, Vec<Chat>>
{
    let repo_channel_parent = &state.channel_parents;

    let ctx_user_id = ctx.user_id_ref();

    repo_channel_parent.get_chats_by_user(ctx_user_id).await
}
