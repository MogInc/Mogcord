use serde::Serialize;

use crate::model::server::Server;

use super::{vec_to_dto, ChannelCreateResponse, ChannelGetResponse, ObjectToDTO};

#[derive(Serialize)]
pub struct ServerCreateResponse
{
    id: String,
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    owner: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<Vec<String>>,
    channels: Vec<ChannelCreateResponse>,
}

impl ObjectToDTO<Server> for ServerCreateResponse
{
    fn obj_to_dto(model_input: Server) -> Self 
    {
        Self
        {
            id: model_input.id,
            r#type: String::from( "Server"),
            name: Some(model_input.name),
            owner: model_input.owner.id,
            users: Some(model_input.users.into_iter().map(|user| user.id).collect()),
            channels: vec_to_dto(model_input.channels),
        }
    }
}

#[derive(Serialize)]
pub struct ServerGetResponse
{
    id: String,
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    owner: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<Vec<String>>,
    channels: Vec<ChannelGetResponse>,
}

impl ObjectToDTO<Server> for ServerGetResponse
{
    fn obj_to_dto(model_input: Server) -> Self 
    {
        Self
        {
            id: model_input.id,
            r#type: String::from( "Server"),
            name: Some(model_input.name),
            owner: model_input.owner.id,
            users: Some(model_input.users.into_iter().map(|user| user.id).collect()),
            channels: vec_to_dto(model_input.channels),
        }
    }
}