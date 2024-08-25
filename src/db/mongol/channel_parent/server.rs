use bson::Uuid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::bubble;
use crate::db::helper;
use crate::model::channel_parent::{Role, Server};
use crate::model::error;

//_id gets an ObjectId signed and will most likely do some voodoo to retrieve a chat
#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::pub_underscore_fields)]
#[allow(clippy::used_underscore_binding)]
pub struct MongolServer
{
    _id: Uuid,
    name: String,
    owner_id: Uuid,
    user_ids: Vec<Uuid>,
    channel_ids: Vec<Uuid>,
    //key is role name
    roles: HashMap<String, Role>,
    //key is user id
    //value is role name
    user_roles: HashMap<String, Vec<String>>,
}

impl TryFrom<&Server> for MongolServer
{
    type Error = error::Server<'static>;

    fn try_from(value: &Server) -> Result<Self, Self::Error>
    {
        let db_id = bubble!(helper::convert_domain_id_to_mongol(&value.id))?;

        let owner_id =
            bubble!(helper::convert_domain_id_to_mongol(&value.owner.id))?;

        let user_ids = value
            .users
            .keys()
            .map(|key| bubble!(helper::convert_domain_id_to_mongol(key)))
            .collect::<Result<_, _>>()?;

        let channel_ids = value
            .channels
            .keys()
            .map(|key| bubble!(helper::convert_domain_id_to_mongol(key)))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            _id: db_id,
            name: value.name.to_string(),
            owner_id,
            user_ids,
            channel_ids,
            roles: value.roles.clone(),
            user_roles: value.user_roles.clone(),
        })
    }
}
