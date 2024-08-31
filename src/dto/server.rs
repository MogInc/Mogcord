use std::collections::HashMap;

use serde::Serialize;

use crate::model::channel_parent::{Role, Server};

use super::{vec_to_dto, ChannelCreateResponse, ChannelGetResponse, ObjectToDTO};

#[derive(Serialize)]
pub struct ServerCreateResponse
{
    pub id: String,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<ChannelCreateResponse>>,
}

impl ObjectToDTO<Server> for ServerCreateResponse
{
    fn obj_to_dto(model_input: Server) -> Self
    {
        Self {
            id: model_input.id,
            r#type: String::from("Server"),
            name: Some(model_input.name),
            owner: Some(model_input.owner.id),
            users: Some(model_input.users.into_keys().collect()),
            channels: Some(vec_to_dto(model_input.channels.into_values().collect())),
        }
    }
}

#[derive(Serialize)]
pub struct ServerGetResponse
{
    pub id: String,
    pub r#type: String,
    pub name: String,
    pub owner: String,
    pub users: Vec<String>,
    pub channels: Vec<ChannelGetResponse>,
    pub roles: Vec<Role>,
    //key user id
    //value role names
    pub user_roles: HashMap<String, Vec<String>>,
}

impl ObjectToDTO<Server> for ServerGetResponse
{
    fn obj_to_dto(model_input: Server) -> Self
    {
        Self {
            id: model_input.id,
            r#type: String::from("Server"),
            name: model_input.name,
            owner: model_input.owner.id,
            users: model_input.users.into_keys().collect(),
            channels: vec_to_dto(model_input.channels.into_values().collect()),
            roles: model_input.roles.into_values().collect(),
            user_roles: model_input.user_roles,
        }
    }
}
