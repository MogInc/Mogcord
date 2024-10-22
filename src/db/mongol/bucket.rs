use mongodb::bson::{
    DateTime,
    Uuid,
};
use serde::{
    Deserialize,
    Serialize,
};

use super::helper::{
    self,
    MongolHelper,
};
use crate::model::bucket::Bucket;
use crate::model::error;
use crate::{
    bubble,
    server_error,
};

#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::pub_underscore_fields)]
#[allow(clippy::used_underscore_binding)]
pub struct MongolBucket
{
    pub _id: Uuid,
    pub channel_id: Uuid,
    pub date: DateTime,
    pub message_ids: Vec<Uuid>,
}

impl TryFrom<&Bucket> for MongolBucket
{
    type Error = error::Server<'static>;

    fn try_from(value: &Bucket) -> Result<Self, Self::Error>
    {
        let bucket_id =
            bubble!(helper::convert_domain_id_to_mongol(&value.id))?;

        let channel_id =
            bubble!(helper::convert_domain_id_to_mongol(&value.channel.id))?;

        let bucket_date = value.date.convert_to_bson_date().map_err(|err| {
            server_error!(
                error::Kind::Parse,
                error::OnType::Date
            )
            .add_debug_info(
                "bucket date",
                value.date.to_string(),
            )
            .add_debug_info("error", err.to_string())
        })?;

        let bucket_message_ids = value
            .messages
            .iter()
            .map(|message| {
                bubble!(helper::convert_domain_id_to_mongol(&message.id))
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            _id: bucket_id,
            channel_id,
            date: bucket_date,
            message_ids: bucket_message_ids,
        })
    }
}
