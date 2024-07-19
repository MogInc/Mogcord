use serde::Serialize;

use crate::model::chat::ChatInfo;

use super::ObjectToDTO;

#[derive(Serialize)]
pub struct ChatInfoCreateResponse
{
    id: String,
    name: Option<String>,
}

impl ObjectToDTO<ChatInfo> for ChatInfoCreateResponse 
{
    fn obj_to_dto(chat: ChatInfo) -> Self
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

impl ObjectToDTO<ChatInfo> for ChatInfoGetResponse 
{
    fn obj_to_dto(chat: ChatInfo) -> Self
    {
        Self
        {
            id: chat.id,
            name: chat.name,
        }
    }
}