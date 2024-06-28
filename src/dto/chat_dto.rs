use serde::Serialize;

use crate::model::chat::Chat;

use super::UserDTO;

#[derive(Serialize)]
pub struct ChatDTO
{
    uuid: String,
    name: Option<String>,
    r#type: String,
    owners: Vec<UserDTO>,
    users: Option<Vec<UserDTO>>,
}

impl ChatDTO
{
    pub fn obj_to_dto(chat: Chat) -> Self
    {
        Self
        {
            uuid: chat.uuid,
            name: chat.name,
            r#type: chat.r#type.to_string(),
            owners: UserDTO::vec_to_dto(chat.owners),
            users: chat.users.map(|users| UserDTO::vec_to_dto(users))
        }
    }
    
    pub fn vec_to_dto(chat: Vec<Chat>) -> Vec<Self>
    {
        let mut chat_dto: Vec<Self> = Vec::new();

        for chat_ in chat
        {
            chat_dto.push(Self::obj_to_dto(chat_))
        }
        
        return chat_dto;
    }
}