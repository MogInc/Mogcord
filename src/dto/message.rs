use serde::Serialize;

use crate::model::message::Message;

use super::ObjectToDTO;

#[derive(Serialize)]
pub struct MessageCreateResponse
{
    pub id: String,
    pub value: String,
    pub timestamp: String,
    pub owner_id: String,
    pub channel_id: String,
    pub bucket_id: String,
    //we actually gonna delete stuff?
    //(:sins:)
    pub flag: String,
}

impl ObjectToDTO<Message> for MessageCreateResponse
{
    fn obj_to_dto(message: Message) -> Self
    {
        Self {
            id: message.id,
            value: message.value,
            timestamp: message.timestamp.to_rfc3339(),
            owner_id: message.owner.id,
            channel_id: message.channel.id,
            bucket_id: message.bucket_id.map_or(String::new(), |bucket| bucket),
            flag: message.flag.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct MessageGetResponse
{
    pub id: String,
    pub value: String,
    pub timestamp: String,
    pub owner_id: String,
    pub channel_id: String,
    pub bucket_id: String,
    //we actually gonna delete stuff?
    //(:sins:)
    pub flag: String,
}

impl ObjectToDTO<Message> for MessageGetResponse
{
    fn obj_to_dto(message: Message) -> Self
    {
        Self {
            id: message.id,
            value: message.value,
            timestamp: message.timestamp.to_rfc3339(),
            owner_id: message.owner.id,
            channel_id: message.channel.id,
            bucket_id: message.bucket_id.map_or(String::new(), |bucket| bucket),
            flag: message.flag.to_string(),
        }
    }
}
