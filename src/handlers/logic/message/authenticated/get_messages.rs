use std::sync::Arc;

use crate::middleware::auth::Ctx;
use crate::model::channel::Parent;
use crate::model::message::Message;
use crate::model::{error, AppState, Pagination};
use crate::server_error;

pub async fn get_messages<'err>(
    state: &Arc<AppState>,
    channel_id: &str,
    ctx: &Ctx,
    pagination: &Pagination,
) -> error::Result<'err, Vec<Message>>
{
    let repo_message = &state.messages;
    let repo_parent = &state.channel_parents;

    let current_user_id = ctx.user_id_ref();

    let chat = repo_parent.get_channel_parent(channel_id).await?;

    if !chat.is_user_part_of_channel_parent(current_user_id)
    {
        return Err(server_error!(error::Kind::NotPartOf, error::OnType::ChannelParent)
            .add_client(error::Client::SERVER_CTX_NOT_PART_OF_SERVER));
    }

    if !chat.can_read(current_user_id, Some(channel_id))?
    {
        return Err(server_error!(error::Kind::NotPartOf, error::OnType::ChannelParent)
            .add_client(error::Client::SERVER_CTX_NOT_PART_OF_SERVER));
    }

    repo_message
        .get_valid_messages(channel_id, pagination)
        .await
}
