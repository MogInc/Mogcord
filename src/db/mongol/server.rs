mod repository;


use std::collections::HashMap;
use bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::model::server::Role;
use crate::model::{error, server::Server};
use crate::db::mongol::helper;

//reason for wrapper
//else _id gets an ObjectId signed and will most likely do some voodoo to retrieve a chat
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
    //key is user id
    roles: HashMap<Uuid, Vec<Role>>,
}

impl TryFrom<&Server> for MongolServer
{
    type Error = error::Server;
    
    fn try_from(value: &Server) -> Result<Self, Self::Error> 
    {
        let db_id = helper::convert_domain_id_to_mongol(&value.id)?;

        let owner_id = helper::convert_domain_id_to_mongol(&value.owner.id)?;

        let user_ids = value
            .users
            .keys()
            .map(|key| helper::convert_domain_id_to_mongol(key))
            .collect::<Result<_, _>>()?;

        let channel_ids = value
            .channels
            .keys()
            .map(|key| helper::convert_domain_id_to_mongol(key))
            .collect::<Result<_, _>>()?;

        let roles = value
            .roles
            .iter()
            .map(|(key, val)| {
                let uuid = helper::convert_domain_id_to_mongol(&key.id)?;
                Ok((uuid, val.clone()))
            })
            .collect::<Result<_, _>>()?;

        Ok(
            Self 
            { 
                _id: db_id,
                name: value.name.to_string(),
                owner_id,
                user_ids,
                channel_ids,
                roles,
            }
        )
    }
}