use mongodb::bson::{DateTime, Uuid};
use serde::{Serialize, Deserialize};


use crate::db::mongoldb::{mongol_helper, MongolHelper};
use crate::model::chat::Bucket;

use super::MongolError;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolBucket 
{
    pub _id: Uuid,
    pub chat_id: Uuid, 
    pub date: DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_ids: Option<Vec<Uuid>>, 
}

impl TryFrom<&Bucket> for MongolBucket
{
    type Error = MongolError;

    fn try_from(value: &Bucket) -> Result<Self, Self::Error> 
    {
        let bucket_id = mongol_helper::convert_domain_id_to_mongol(&value.id)?;

        let chat_id = mongol_helper::convert_domain_id_to_mongol(&value.chat.id)?;

        let bucket_date = value
            .date
            .convert_to_bson_date()
            .map_err(|_| MongolError::FailedDateParsing)?;

        let bucket_message_ids = value
            .messages.as_ref()
            .map(|messages|{
                messages.into_iter().map(|message|{
                    mongol_helper::convert_domain_id_to_mongol(&message.id)
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