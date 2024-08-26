mod repository;

use bson::Bson;
use mongodb::bson::{DateTime, Uuid};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use super::helper::{self, as_string};
use crate::bubble;
use crate::model::error;
use crate::model::message::{self, Message};

#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::pub_underscore_fields)]
#[allow(clippy::used_underscore_binding)]
pub struct MongolMessage
{
    pub _id: Uuid,
    pub value: String,
    pub timestamp: DateTime,
    pub owner_id: Uuid,
    pub channel_id: Uuid,
    pub bucket_id: Option<Uuid>,
    #[serde(serialize_with = "as_string")]
    pub flag: message::Flag,
}

impl TryFrom<&Message> for MongolMessage
{
    type Error = error::Server<'static>;

    fn try_from(value: &Message) -> Result<Self, Self::Error>
    {
        let message_id = bubble!(helper::convert_domain_id_to_mongol(&value.id))?;

        let owner_id = bubble!(helper::convert_domain_id_to_mongol(&value.owner.id))?;

        let channel_id = bubble!(helper::convert_domain_id_to_mongol(&value.channel.id))?;

        let bucket_id_option = value
            .bucket_id
            .as_ref()
            .map(|bucket_id| helper::convert_domain_id_to_mongol(bucket_id))
            .transpose()?;

        let timestamp: SystemTime = value.timestamp.into();

        Ok(Self {
            _id: message_id,
            value: value.value.clone(),
            timestamp: DateTime::from(timestamp),
            owner_id,
            channel_id,
            bucket_id: bucket_id_option,
            flag: value.flag.clone(),
        })
    }
}

impl From<message::Flag> for Bson
{
    fn from(message_flag: message::Flag) -> Bson
    {
        Bson::String(message_flag.to_string())
    }
}
