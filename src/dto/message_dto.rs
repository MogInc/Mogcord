use serde::Serialize;

use crate::model::message::Message;

#[derive(Serialize)]
pub struct MessageDTO
{
    id: String,
    value: String,
    timestamp: String,
    owner_id: String,
    chat_id: String,
    bucket_id: String,
    //we actually gonna delete stuff?
    //(:sins:)
    flag: String,
}

impl MessageDTO
{
    pub fn obj_to_dto(message: Message) -> Self
    {
        Self
        {
            id: message.id,
            value: message.value,
            timestamp: message.timestamp.to_rfc3339(),
            owner_id: message.owner.id,
            chat_id: message.chat.id,
            bucket_id: message.bucket_id.map_or(String::from(""), |bucket| bucket),
            flag: message.flag.to_string(),
        }
    }
    
    pub fn vec_to_dto(messages: Vec<Message>) -> Vec<Self>
    {
        let mut messages_dto: Vec<Self> = Vec::new();

        for message in messages
        {
            messages_dto.push(Self::obj_to_dto(message))
        }
        
        return messages_dto;
    }
}