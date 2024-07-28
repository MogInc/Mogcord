use bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::{bubble, db::helper, model::{channel_parent::chat::Group, error}};

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
    pub channel_id: Uuid,
}

impl TryFrom<&Group> for MongolGroup
{
    type Error = error::Server<'static>;

    fn try_from(value: &Group) -> Result<Self, Self::Error> 
    {
        let db_id = bubble!(helper::convert_domain_id_to_mongol(&value.id))?;

        let channel_id = bubble!(helper::convert_domain_id_to_mongol(&value.channel.id))?;

        let owner_id = bubble!(helper::convert_domain_id_to_mongol(&value.owner.id))?;

        let user_ids = value.users
            .keys()
            .map(|key| bubble!(helper::convert_domain_id_to_mongol(key)))
            .collect::<Result<_, _>>()?;

        Ok(
            Self
            {
                _id: db_id,
                name: value.name.to_string(),
                owner_id,
                user_ids,
                channel_id,
            }
        )
    }
}