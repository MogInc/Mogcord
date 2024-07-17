use serde::Serialize;

use crate::model::chat::Chat;

use super::{vec_to_dto, ChatInfoDTO, ObjectToDTO};

#[derive(Serialize)]
pub struct ChatDTO
{
    id: String,
    name: Option<String>,
    owners: Vec<String>,
    users: Vec<String>,
    chat_info: Vec<ChatInfoDTO>,
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
                    name: None,
                    owners: owners.into_iter().map(|user| user.id).collect(),
                    users: Vec::new(),
                    chat_info: vec![ChatInfoDTO::obj_to_dto(chat_info)],
                }
            },
            Chat::Group { id, name, owner, users, chat_info } => 
            {
                Self
                {
                    id,
                    name: Some(name),
                    owners: vec![owner.id],
                    users: users.into_iter().map(|user| user.id).collect(),
                    chat_info: vec![ChatInfoDTO::obj_to_dto(chat_info)],
                }
            },
            Chat::Server { id, name, owner, users, chat_infos } => 
            {
                Self
                {
                    id,
                    name: Some(name),
                    owners: vec![owner.id],
                    users: users.into_iter().map(|user| user.id).collect(),
                    chat_info: vec_to_dto(chat_infos)
                }
            },
        }
    }
}