use serde::Serialize;

use crate::model::channel_parent::chat::Chat;

use super::{ChannelCreateResponse, ChannelGetResponse, ObjectToDTO};

#[derive(Serialize)]
pub struct ChatCreateResponse
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
}

impl ObjectToDTO<Chat> for ChatCreateResponse
{
    fn obj_to_dto(model_input: Chat) -> Self
    {
        match model_input
        {
            Chat::Private(private) => Self {
                id: private.id,
                r#type: String::from("Private"),
                name: None,
                owner: None,
                owners: Some(
                    private.owners.into_iter().map(|user| user.id).collect(),
                ),
                users: None,
                channel: Some(
                    ChannelCreateResponse::obj_to_dto(private.channel),
                ),
            },
            Chat::Group(group) => Self {
                id: group.id,
                r#type: String::from("Group"),
                name: Some(group.name),
                owner: Some(group.owner.id),
                owners: None,
                users: Some(group.users.into_keys().collect()),
                channel: Some(ChannelCreateResponse::obj_to_dto(group.channel)),
            },
        }
    }
}

#[derive(Serialize)]
pub struct ChatGetResponse
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
}

impl ObjectToDTO<Chat> for ChatGetResponse
{
    fn obj_to_dto(model_input: Chat) -> Self
    {
        match model_input
        {
            Chat::Private(private) => Self {
                id: private.id,
                r#type: String::from("Private"),
                name: None,
                owner: None,
                owners: Some(
                    private.owners.into_iter().map(|user| user.id).collect(),
                ),
                users: None,
                channel: Some(ChannelGetResponse::obj_to_dto(private.channel)),
            },
            Chat::Group(group) => Self {
                id: group.id,
                r#type: String::from("Group"),
                name: Some(group.name),
                owner: Some(group.owner.id),
                owners: None,
                users: Some(group.users.into_keys().collect()),
                channel: Some(ChannelGetResponse::obj_to_dto(group.channel)),
            },
        }
    }
}
