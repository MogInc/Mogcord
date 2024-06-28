use serde::Serialize;

use crate::model::chat::Chat;

#[derive(Serialize)]
pub struct ChatDTO
{
    uuid: String,
    name: Option<String>,
    r#type: String,
    owners: Vec<String>,
    users: Option<Vec<String>>,
}

impl ChatDTO
{
    pub fn obj_to_dto(chat: Chat) -> Self
    {
        let owner_ids : Vec<String> = chat
            .owners
            .into_iter()
            .map(|owner| owner.uuid)
            .collect();

        let user_ids : Option<Vec<String>> = chat
            .users
            .map(|users|{
                users.into_iter()
                .map(|user| user.uuid)
                .collect()
            });

        Self
        {
            uuid: chat.uuid,
            name: chat.name,
            r#type: chat.r#type.to_string(),
            owners: owner_ids,
            users: user_ids
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