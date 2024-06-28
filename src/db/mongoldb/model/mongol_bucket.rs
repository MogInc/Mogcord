use mongodb::bson::{DateTime, Uuid};
use serde::{Serialize, Deserialize};


use crate::db::mongoldb::MongolHelper;
use crate::model::chat::Bucket;

use super::MongolError;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolBucket 
{
    pub _id: Uuid,
    pub chat_id: Uuid, 
    pub date: DateTime,
    pub message_ids: Option<Vec<Uuid>>, 
}

impl TryFrom<Bucket> for MongolBucket
{
    type Error = MongolError;

    fn try_from(value: Bucket) -> Result<Self, Self::Error> 
    {
        let bucket_id = Uuid::parse_str(value.id)
            .map_err(|_| MongolError::InvalidID)?;

        let chat_id = Uuid::parse_str(value.chat.id)
            .map_err(|_| MongolError::InvalidID)?;

        let bucket_date = value
            .date
            .convert_to_bson_datetime()
            .map_err(|_| MongolError::FailedDateParsing)?;

        let bucket_message_ids = value
            .messages
            .map(|messages|{
                messages.into_iter().map(|message|{
                    Uuid::parse_str(message.id)
                        .map_err(|_| MongolError::InvalidID)
                }).collect::<Result<_, _>>()
            }).transpose()?;

        Ok(
            Self
            {
                _id: bucket_id,
                chat_id,
                date: bucket_date,
                message_ids: bucket_message_ids,
            }
        )
    }
}