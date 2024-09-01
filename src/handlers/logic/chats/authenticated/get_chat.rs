use std::sync::Arc;

use crate::middleware::auth::Ctx;
use crate::model::channel_parent::chat::Chat;
use crate::model::{error, AppState};
use crate::server_error;

pub async fn get_chat<'err>(
    state: &Arc<AppState>,
    ctx: &Ctx,
    chat_id: &str,
) -> error::Result<'err, Chat>
{
    let repo_chat = &state.chats;

    let chat = repo_chat.get_chat_by_id(chat_id).await?;

    let ctx_user_id = ctx.user_id_ref();

    if !chat.is_user_part_of_chat(ctx_user_id)
    {
        return Err(server_error!(error::Kind::NotPartOf, error::OnType::Chat)
            .add_client(error::Client::CHAT_CTX_NOT_PART_OF_CHAT));
    }

    Ok(chat)
}
