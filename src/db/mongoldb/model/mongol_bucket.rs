use chrono::Datelike;
use mongodb::bson::{self, DateTime, Uuid};
use serde::{Serialize, Deserialize};

use crate::model::{chat::Bucket, message};

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
        let bucket_uuid = Uuid::parse_str(value.uuid)
            .map_err(|_| MongolError::InvalidUUID)?;

        let chat_uuid = Uuid::parse_str(value.chat.uuid)
            .map_err(|_| MongolError::InvalidUUID)?;

        let bucket_datetime = value.date;

        let bucket_date = bson::DateTime::builder()
            .year(bucket_datetime.year())
            .month(bucket_datetime.month().try_into().unwrap())
            .day(bucket_datetime.day().try_into().unwrap())
            .build()
            .map_err(|_| MongolError::FailedDateParsing)?;

        let bucket_message_uuids = value
            .messages
            .map(|messages|{
                messages.into_iter().map(|message|{
                    Uuid::parse_str(message.uuid)
                        .map_err(|_| MongolError::InvalidUUID)
                }).collect::<Result<_, _>>()
            }).transpose()?;

        Ok(
            Self
            {
                _id: bucket_uuid,
                chat_id: chat_uuid,
                date: bucket_date,
                message_ids: bucket_message_uuids,
            }
        )
    }
}