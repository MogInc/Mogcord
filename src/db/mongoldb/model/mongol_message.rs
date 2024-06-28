use std::time::SystemTime;

use mongodb::bson::{DateTime, Uuid};
use serde::{Serialize, Deserialize};

use crate::{db::mongoldb::mongol_helper, model::message::{Message, MessageFlag}};

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
        let message_id = mongol_helper::convert_domain_id_to_mongol(&value.id)?;

        let owner_id = mongol_helper::convert_domain_id_to_mongol(&value.owner.id)?;

        let chat_id = mongol_helper::convert_domain_id_to_mongol(&value.chat.id)?;

        let bucket_id_option = value.bucket_id.map(|bucket_id|
            mongol_helper::convert_domain_id_to_mongol(&bucket_id)
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
