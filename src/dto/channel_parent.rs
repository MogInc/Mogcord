use serde::Serialize;

use crate::model::channel_parent::ChannelParent;

use super::{vec_to_dto, ChannelCreateResponse, ChannelGetResponse, ObjectToDTO};

#[derive(Serialize)]
pub struct ChannelWrapperCreateResponse
{
    id: String,
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owners: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<ChannelCreateResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channels: Option<Vec<ChannelCreateResponse>>,
}

impl ObjectToDTO<ChannelParent> for ChannelWrapperCreateResponse
{
    fn obj_to_dto(model_input: ChannelParent) -> Self 
    {
        match model_input
        {
            ChannelParent::Private(private) => 
            {
                Self
                {
                    id: private.id, 
                    r#type: String::from("Private"),
                    name: None,
                    owner: None,
                    owners: Some(private.owners.iter().map(|user| user.id).collect()),
                    users: None,
                    channel: Some(ChannelCreateResponse::obj_to_dto(private.channel)),
                    channels: None,
                }
            },
            ChannelParent::Group(group) => 
            {
                Self
                {
                    id: group.id,
                    r#type: String::from("Group"),
                    name: Some(group.name),
                    owner: Some(group.owner.id),
                    owners: None,
                    users: Some(group.users().iter().map(|user| user.id).collect()),
                    channel: Some(ChannelCreateResponse::obj_to_dto(group.channel)),
                    channels: None,
                }
            },
            ChannelParent::Server(server) =>
            {
                Self
                {
                    id: server.id,
                    r#type: String::from("Server"),
                    name: Some(server.name),
                    owner: Some(server.owner.id),
                    owners: None,
                    users: Some(server.users().iter().map(|user| user.id).collect()),
                    channel: None,
                    channels: Some(vec_to_dto(server.channels())),
                }
            },
        }
    }
}

#[derive(Serialize)]
pub struct ChannelWrapperGetResponse
{
    id: String,
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owners: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<ChannelGetResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channels: Option<Vec<ChannelGetResponse>>,
}

impl ObjectToDTO<ChannelParent> for ChannelWrapperGetResponse
{
    fn obj_to_dto(model_input: ChannelParent) -> Self 
    {
        match model_input
        {
            ChannelParent::Private(private) => 
            {
                Self
                {
                    id: private.id, 
                    r#type: String::from("Private"),
                    name: None,
                    owner: None,
                    owners: Some(private.owners.iter().map(|user| user.id).collect()),
                    users: None,
                    channel: Some(ChannelGetResponse::obj_to_dto(private.channel)),
                    channels: None,
                }
            },
            ChannelParent::Group(group) => 
            {
                Self
                {
                    id: group.id,
                    r#type: String::from("Group"),
                    name: Some(group.name),
                    owner: Some(group.owner.id),
                    owners: None,
                    users: Some(group.users().iter().map(|user| user.id).collect()),
                    channel: Some(ChannelGetResponse::obj_to_dto(group.channel)),
                    channels: None,
                }
            },
            ChannelParent::Server(server) =>
            {
                Self
                {
                    id: server.id,
                    r#type: String::from("Server"),
                    name: Some(server.name),
                    owner: Some(server.owner.id),
                    owners: None,
                    users: Some(server.users().iter().map(|user| user.id).collect()),
                    channel: None,
                    channels: Some(vec_to_dto(server.channels())),
                }
            },
        }
    }
}