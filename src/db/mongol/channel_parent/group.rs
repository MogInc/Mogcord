use bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::{db::{helper, MongolChannel}, model::{channel_parent::Group, error}};

//_id gets an ObjectId signed and will most likely do some voodoo to retrieve a chat
#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::pub_underscore_fields)]
#[allow(clippy::used_underscore_binding)]
pub struct MongolGroup
{
    pub _id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub user_ids: Vec<Uuid>,
    pub channel: MongolChannel,
}

impl TryFrom<&Group> for MongolGroup
{
    type Error = error::Server;

    fn try_from(value: &Group) -> Result<Self, Self::Error> 
    {
        let db_id = helper::convert_domain_id_to_mongol(&value.id)?;

        let owner_id = helper::convert_domain_id_to_mongol(&value.owner.id)?;

        let user_ids = value.users
            .iter()
            .map(|(key, _)| helper::convert_domain_id_to_mongol(key))
            .collect::<Result<_, _>>()?;


        let group = Self
        {
            _id: db_id,
            name: value.name.to_string(),
            owner_id,
            user_ids,
            channel: MongolChannel::try_from(&value.channel)?,
        };

        Ok(group)
    }
}