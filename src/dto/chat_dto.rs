use serde::Serialize;

use crate::model::chat::Chat;

use super::{vec_to_dto, ChatInfoDTO, ObjectToDTO};

#[derive(Serialize)]
pub struct ChatDTO
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
    chat_info: Option<ChatInfoDTO>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_infos: Option<Vec<ChatInfoDTO>>,
}

impl ObjectToDTO<Chat> for ChatDTO
{
    fn obj_to_dto(model_input: Chat) -> Self 
    {
        match model_input
        {
            Chat::Private { id, owners, chat_info } => 
            {
                Self
                {
                    id, 
                    r#type: String::from( "Private"),
                    name: None,
                    owner: None,
                    owners: Some(owners.into_iter().map(|user| user.id).collect()),
                    users: None,
                    chat_info: Some(ChatInfoDTO::obj_to_dto(chat_info)),
                    chat_infos: None,
                }
            },
            Chat::Group { id, name, owner, users, chat_info } => 
            {
                Self
                {
                    id,
                    r#type: String::from( "Group"),
                    name: Some(name),
                    owner: Some(owner.id),
                    owners: None,
                    users: Some(users.into_iter().map(|user| user.id).collect()),
                    chat_info: Some(ChatInfoDTO::obj_to_dto(chat_info)),
                    chat_infos: None,
                }
            },
            Chat::Server { id, name, owner, users, chat_infos } => 
            {
                Self
                {
                    id,
                    r#type: String::from( "Server"),
                    name: Some(name),
                    owner: Some(owner.id),
                    owners: None,
                    users: Some(users.into_iter().map(|user| user.id).collect()),
                    chat_info: None,
                    chat_infos: Some(vec_to_dto(chat_infos))
                }
            },
        }
    }
}