use serde::Serialize;

use crate::model::chat::ChatInfo;

use super::ObjectToDTO;

#[derive(Serialize)]
pub struct ChatInfoDTO
{
    id: String,
    name: Option<String>,
}

impl ObjectToDTO<ChatInfo> for ChatInfoDTO 
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