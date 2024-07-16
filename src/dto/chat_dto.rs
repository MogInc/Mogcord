use serde::Serialize;

use crate::model::chat::Chat;

use super::{ChatTypeDTO, ObjectToDTO};

#[derive(Serialize)]
pub struct ChatDTO
{
    id: String,
    name: Option<String>,
    r#type: ChatTypeDTO,
}

impl ObjectToDTO<Chat> for ChatDTO 
{
    fn obj_to_dto(chat: Chat) -> Self
    {
        Self
        {
            id: chat.id,
            name: chat.name,
            r#type: ChatTypeDTO::obj_to_dto(chat.r#type),
        }
    }
}