use mongodb::bson::{Bson, Uuid};
use serde::{Serialize, Deserialize};

use crate::model::message::{Message, MessageFlag};

use super::MongolError;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolMessage
{
    pub _id : Uuid,
    pub owner_id: Uuid,
    pub value: String,
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

        let bucket_uuid = value.bucket.map(|bucket|
            Uuid::parse_str(bucket.uuid).map_err(|_| MongolError::InvalidUUID)
        ).transpose()?;

        Ok(
            Self
            { 
                _id: message_uuid, 
                owner_id: owner_uuid, 
                value: value.value, 
                chat_id: chat_uuid,
                bucket_id: bucket_uuid,
                flag: value.flag
            }
        )
    }
}
