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
        let message_id = Uuid::parse_str(value.id)
            .map_err(|_| MongolError::InvalidID)?;

        let owner_id = Uuid::parse_str(value.owner.id)
            .map_err(|_| MongolError::InvalidID)?;

        let chat_id = Uuid::parse_str(value.chat.id)
            .map_err(|_| MongolError::InvalidID)?;

        let bucket_id_option = value.bucket_id.map(|bucket_id|
            Uuid::parse_str(bucket_id).map_err(|_| MongolError::InvalidID)
        ).transpose()?;

        let timestamp: SystemTime = value.timestamp.into();

        Ok(
            Self
            { 
                _id: message_id, 
                value: value.value, 
                timestamp: DateTime::from(timestamp),
                owner_id, 
                chat_id,
                bucket_id: bucket_id_option,
                flag: value.flag
            }
        )
    }
}
