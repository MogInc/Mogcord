use mongodb::bson::{DateTime, Uuid};
use serde::{Serialize, Deserialize};


use crate::db::mongoldb::{mongol_helper, MongolHelper};
use crate::model::chat::Bucket;
use crate::model::misc::ServerError;


#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::pub_underscore_fields)]
#[allow(clippy::used_underscore_binding)]
pub struct MongolBucket 
{
    pub _id: Uuid,
    pub chat_id: Uuid, 
    pub date: DateTime,
    pub message_ids: Vec<Uuid>, 
}

impl TryFrom<&Bucket> for MongolBucket
{
    type Error = ServerError;

    fn try_from(value: &Bucket) -> Result<Self, Self::Error> 
    {
        let bucket_id = mongol_helper::convert_domain_id_to_mongol(&value.id)?;

        let chat_id = mongol_helper::convert_domain_id_to_mongol(&value.chat.id)?;

        let bucket_date = value
            .date
            .convert_to_bson_date()
            .map_err(|_| ServerError::FailedDateParsing)?;

        let bucket_message_ids = value
            .messages
            .iter()
            .map(|message|mongol_helper::convert_domain_id_to_mongol(&message.id))
            .collect::<Result<_,_>>()?;

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