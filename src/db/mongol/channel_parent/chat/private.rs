use bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::{db::{helper, MongolChannel, ParentType}, model::{channel_parent::chat::Private, error}};

//_id gets an ObjectId signed and will most likely do some voodoo to retrieve a chat
#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::pub_underscore_fields)]
#[allow(clippy::used_underscore_binding)]
pub struct MongolPrivate
{
    pub _id: Uuid,
    pub owner_ids: Vec<Uuid>,
    pub channel: MongolChannel
}

impl TryFrom<&Private> for MongolPrivate
{
    type Error = error::Server;

    fn try_from(value: &Private) -> Result<Self, Self::Error> 
    {
        let db_id = helper::convert_domain_id_to_mongol(&value.id)?;

        let owner_ids = value.owners
            .iter()
            .map(|owner| helper::convert_domain_id_to_mongol(&owner.id))
            .collect::<Result<_, _>>()?;

        Ok(
            Self 
            { 
                _id: db_id,
                owner_ids,
                channel: MongolChannel::try_from((&value.channel, ParentType::ChatPrivate))?
            }
        )
    }
}