use std::time::SystemTime;

use bson::Bson;
use mongodb::bson::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

use crate::{db::mongoldb::{as_string, mongol_helper}, model::{message::{Message, MessageFlag}, misc::ServerError}};


#[derive(Debug, Serialize, Deserialize)]
pub struct MongolMessage
{
    pub _id : Uuid,
    pub value: String,
    pub timestamp: DateTime,
    pub owner_id: Uuid,
    pub chat_id: Uuid,
    pub bucket_id: Option<Uuid>,
    #[serde(serialize_with = "as_string")]
    pub flag: MessageFlag
}

impl TryFrom<&Message> for MongolMessage
{
    type Error = ServerError;

    fn try_from(value: &Message) -> Result<Self, Self::Error>
    {     
        let message_id = mongol_helper::convert_domain_id_to_mongol(&value.id)?;
        
        let owner_id = mongol_helper::convert_domain_id_to_mongol(&value.owner.id)?;
        
        let chat_id = mongol_helper::convert_domain_id_to_mongol(&value.chat.id)?;

        let bucket_id_option = value.bucket_id
            .as_ref()
            .map(|bucket_id|mongol_helper::convert_domain_id_to_mongol(&bucket_id))
            .transpose()?;

        let timestamp: SystemTime = value.timestamp.into();

        Ok(
            Self
            { 
                _id: message_id, 
                value: value.value.clone(), 
                timestamp: DateTime::from(timestamp),
                owner_id, 
                chat_id,
                bucket_id: bucket_id_option,
                flag: value.flag.clone(),
            }
        )
    }
}

impl From<MessageFlag> for Bson 
{
    fn from(message_flag: MessageFlag) -> Bson 
    {
        Bson::String(message_flag.to_string())
    }
}