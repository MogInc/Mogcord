use std::sync::Arc;
use axum::{extract::{self, Path, State}, response::IntoResponse, Json};
use serde::Deserialize;

use crate::model::{channel::Parent, error, AppState};
use crate::middleware::auth::Ctx;
use crate::dto::{MessageCreateResponse, ObjectToDTO};

#[derive(Deserialize)]
pub struct UpdateMessageRequest
{
    value: String,
}
pub async fn update_message(
    State(state, ): State<Arc<AppState>>,
    Path((channel_id, message_id)): Path<(String, String)>,
    ctx: Ctx,
    extract::Json(payload): extract::Json<UpdateMessageRequest>,
) -> impl IntoResponse
{
    let repo_message = &state.messages;
    let repo_parent = &state.channel_parents;

    let ctx_user_id = ctx.user_id_ref();
    
    let mut message = repo_message
        .get_message(&message_id)
        .await?;
    
    if !message.is_channel_part_of_message(&channel_id)
    {
        return Err(error::Server::new(
            error::Kind::NotPartOf,
            error::OnType::Channel,
            file!(),
            line!())
            .add_client(error::Client::MESSAGE_NOT_PART_CHANNEL)
        );
    }

    let channel_parent = repo_parent
        .get_channel_parent(&channel_id)
        .await?;

    let user_roles = channel_parent.get_user_roles(ctx_user_id);

    if !message.update_value(payload.value, ctx_user_id, user_roles)?
    {
        return Err(error::Server::new(
            error::Kind::NoChange,
            error::OnType::Message,
            file!(),
            line!())
            .add_client(error::Client::MESSAGE_NOT_PART_CHANNEL)
        );
    }

    match repo_message.update_message(message).await
    {
        Ok(message) =>  Ok(Json(MessageCreateResponse::obj_to_dto(message))),
        Err(err) => Err(err),
    }
}