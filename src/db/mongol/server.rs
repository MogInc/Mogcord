mod repository;


use bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::model::{error, server::Server};
use crate::db::mongol::helper;
use super::{MongolChannelWrapper, MongolChannel};

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
    channels: Vec<MongolChannel> 
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
            .iter()
            .map(|owner| helper::convert_domain_id_to_mongol(&owner.id))
            .collect::<Result<_, _>>()?;

        Ok(
            Self 
            { 
                _id: db_id,
                name: value.name.to_string(),
                owner_id,
                user_ids,
                channels:  MongolChannelWrapper::try_from(&value.channels)?.0,
            }
        )
    }
}