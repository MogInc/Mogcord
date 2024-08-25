use axum::extract::{
    self,
    Path,
    State,
};
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

use crate::dto::{
    MessageCreateResponse,
    ObjectToDTO,
};
use crate::middleware::auth::Ctx;
use crate::model::channel::Parent;
use crate::model::message::Message;
use crate::model::{
    error,
    AppState,
};
use crate::server_error;

#[derive(Deserialize)]
pub struct CreateMessageRequest
{
    value: String,
}
pub async fn create_message(
    State(state): State<Arc<AppState>>,
    Path(channel_id): Path<String>,
    ctx: Ctx,
    extract::Json(payload): extract::Json<CreateMessageRequest>,
) -> impl IntoResponse
{
    let repo_message = &state.messages;
    let repo_user = &state.users;
    let repo_parent = &state.channel_parents;

    let ctx_user_id = ctx.user_id_ref();

    let channel_parent = repo_parent.get_channel_parent(&channel_id).await?;

    if !channel_parent.can_write(ctx_user_id, Some(&channel_id))?
    {
        return Err(server_error!(
            error::Kind::NotAllowed,
            error::OnType::ChannelParent
        )
        .add_client(error::Client::MESSAGE_CREATE_FAIL));
    }

    let owner = repo_user.get_user_by_id(ctx_user_id).await?;

    let channel = channel_parent.get_channel(Some(&channel_id))?;

    let message = Message::new(
        payload.value,
        owner,
        channel.clone(),
    );

    match repo_message.create_message(message).await
    {
        Ok(message) => Ok(Json(
            MessageCreateResponse::obj_to_dto(message),
        )),
        Err(err) => Err(err),
    }
}
