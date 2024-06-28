use serde::Serialize;

use crate::model::message::Message;

#[derive(Serialize)]
pub struct MessageDTO
{
    uuid: String,
    value: String,
    timestamp: String,
    owner_uuid: String,
    chat_uuid: String,
    bucket_uuid: String,
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
            uuid: message.uuid,
            value: message.value,
            timestamp: message.timestamp.to_rfc3339(),
            owner_uuid: message.owner.uuid,
            chat_uuid: message.chat.uuid,
            bucket_uuid: message.bucket_uuid.map_or(String::from(""), |bucket| bucket),
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