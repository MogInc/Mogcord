use serde::Serialize;

use crate::model::chat;

use super::ObjectToDTO;

#[derive(Serialize)]
pub struct ChatInfoCreateResponse
{
    id: String,
    name: Option<String>,
}

impl ObjectToDTO<chat::Info> for ChatInfoCreateResponse 
{
    fn obj_to_dto(chat: chat::Info) -> Self
    {
        Self
        {
            id: chat.id,
            name: chat.name,
        }
    }
}

#[derive(Serialize)]
pub struct ChatInfoGetResponse
{
    id: String,
    name: Option<String>,
}

impl ObjectToDTO<chat::Info> for ChatInfoGetResponse 
{
    fn obj_to_dto(chat: chat::Info) -> Self
    {
        Self
        {
            id: chat.id,
            name: chat.name,
        }
    }
}