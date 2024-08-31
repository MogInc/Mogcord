use serde::Serialize;

use crate::model::channel_parent::chat::Chat;

use super::{ChannelCreateResponse, ChannelGetResponse, ObjectToDTO};

#[derive(Serialize)]
pub struct ChatCreateResponse
{
    pub id: String,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<ChannelCreateResponse>,
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
                owners: Some(private.owners.into_iter().map(|user| user.id).collect()),
                users: None,
                channel: Some(ChannelCreateResponse::obj_to_dto(private.channel)),
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
    pub id: String,
    pub r#type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<ChannelGetResponse>,
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
                name: String::new(),
                owner: None,
                owners: Some(private.owners.into_iter().map(|user| user.id).collect()),
                users: None,
                channel: Some(ChannelGetResponse::obj_to_dto(private.channel)),
            },
            Chat::Group(group) => Self {
                id: group.id,
                r#type: String::from("Group"),
                name: group.name,
                owner: Some(group.owner.id),
                owners: None,
                users: Some(group.users.into_keys().collect()),
                channel: Some(ChannelGetResponse::obj_to_dto(group.channel)),
            },
        }
    }

    fn obj_to_dto_with_user(model_input: Chat, current_user_id: &str) -> Self
    {
        match model_input
        {
            Chat::Private(private) =>
            {
                let owners: Vec<String> =
                    private.owners.iter().map(|user| user.id.clone()).collect();

                let name = private
                    .owners
                    .iter()
                    .filter(|&owner| owner.id != current_user_id)
                    .cloned()
                    .map(|owner| owner.username)
                    .collect::<Vec<String>>()
                    .join(", ");

                Self {
                    id: private.id,
                    r#type: String::from("Private"),
                    name,
                    owner: None,
                    owners: Some(owners),
                    users: None,
                    channel: Some(ChannelGetResponse::obj_to_dto(private.channel)),
                }
            }
            Chat::Group(_) => Self::obj_to_dto(model_input),
        }
    }
}
