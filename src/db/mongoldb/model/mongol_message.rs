use std::time::SystemTime;

use mongodb::bson::{DateTime, Uuid};
use serde::{Serialize, Deserialize};

use crate::model::message::{Message, MessageFlag};

use super::MongolError;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolMessage
{
    pub _id : Uuid,
    pub value: String,
    pub timestamp: DateTime,
    pub owner_id: Uuid,
    pub chat_id: Uuid,
    pub bucket_id: Option<Uuid>,
    pub flag: MessageFlag
}

impl TryFrom<Message> for MongolMessage
{
    type Error = MongolError;

    fn try_from(value: Message) -> Result<Self, Self::Error>
    {
        let message_uuid = Uuid::parse_str(value.uuid)
            .map_err(|_| MongolError::InvalidUUID)?;

        let owner_uuid = Uuid::parse_str(value.owner.uuid)
            .map_err(|_| MongolError::InvalidUUID)?;

        let chat_uuid = Uuid::parse_str(value.chat.uuid)
            .map_err(|_| MongolError::InvalidUUID)?;

        let bucket_uuid_option = value.bucket_uuid.map(|bucket_uuid|
            Uuid::parse_str(bucket_uuid).map_err(|_| MongolError::InvalidUUID)
        ).transpose()?;

        let timestamp: SystemTime = value.timestamp.into();

        Ok(
            Self
            { 
                _id: message_uuid, 
                value: value.value, 
                timestamp: DateTime::from(timestamp),
                owner_id: owner_uuid, 
                chat_id: chat_uuid,
                bucket_id: bucket_uuid_option,
                flag: value.flag
            }
        )
    }
}
