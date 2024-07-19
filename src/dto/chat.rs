use serde::Serialize;

use crate::model::chat::Chat;

use super::{vec_to_dto, ChatInfoCreateResponse, ChatInfoGetResponse, ObjectToDTO};

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
    chat_info: Option<ChatInfoCreateResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_infos: Option<Vec<ChatInfoCreateResponse>>,
}

impl ObjectToDTO<Chat> for ChatCreateResponse
{
    fn obj_to_dto(model_input: Chat) -> Self 
    {
        match model_input
        {
            Chat::Private(private) => 
            {
                Self
                {
                    id: private.id, 
                    r#type: String::from( "Private"),
                    name: None,
                    owner: None,
                    owners: Some(private.owners.into_iter().map(|user| user.id).collect()),
                    users: None,
                    chat_info: Some(ChatInfoCreateResponse::obj_to_dto(private.chat_info)),
                    chat_infos: None,
                }
            },
            Chat::Group(group) => 
            {
                Self
                {
                    id: group.id,
                    r#type: String::from( "Group"),
                    name: Some(group.name),
                    owner: Some(group.owner.id),
                    owners: None,
                    users: Some(group.users.into_iter().map(|user| user.id).collect()),
                    chat_info: Some(ChatInfoCreateResponse::obj_to_dto(group.chat_info)),
                    chat_infos: None,
                }
            },
            Chat::Server(server) => 
            {
                Self
                {
                    id: server.id,
                    r#type: String::from( "Server"),
                    name: Some(server.name),
                    owner: Some(server.owner.id),
                    owners: None,
                    users: Some(server.users.into_iter().map(|user| user.id).collect()),
                    chat_info: None,
                    chat_infos: Some(vec_to_dto(server.chat_infos))
                }
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
    chat_info: Option<ChatInfoGetResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_infos: Option<Vec<ChatInfoGetResponse>>,
}

impl ObjectToDTO<Chat> for ChatGetResponse
{
    fn obj_to_dto(model_input: Chat) -> Self 
    {
        match model_input
        {
            Chat::Private(private) => 
            {
                Self
                {
                    id: private.id, 
                    r#type: String::from("Private"),
                    name: None,
                    owner: None,
                    owners: Some(private.owners.into_iter().map(|user| user.id).collect()),
                    users: None,
                    chat_info: Some(ChatInfoGetResponse::obj_to_dto(private.chat_info)),
                    chat_infos: None,
                }
            },
            Chat::Group(group) => 
            {
                Self
                {
                    id: group.id,
                    r#type: String::from("Group"),
                    name: Some(group.name),
                    owner: Some(group.owner.id),
                    owners: None,
                    users: Some(group.users.into_iter().map(|user| user.id).collect()),
                    chat_info: Some(ChatInfoGetResponse::obj_to_dto(group.chat_info)),
                    chat_infos: None,
                }
            },
            Chat::Server(server) => 
            {
                Self
                {
                    id: server.id,
                    r#type: String::from( "Server"),
                    name: Some(server.name),
                    owner: Some(server.owner.id),
                    owners: None,
                    users: Some(server.users.into_iter().map(|user| user.id).collect()),
                    chat_info: None,
                    chat_infos: Some(vec_to_dto(server.chat_infos))
                }
            },
        }
    }
}