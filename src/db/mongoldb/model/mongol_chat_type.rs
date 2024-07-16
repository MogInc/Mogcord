use bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::{db::mongoldb::mongol_helper, model::{chat::ChatType, misc::ServerError}};

#[derive(Debug, Serialize, Deserialize)]
pub enum MongolChatType
{
    Private
    {
        owner_ids: Vec<Uuid>
    },
    Group
    {
        name: String,
        owner_id: Uuid,
        user_ids: Vec<Uuid>,
    },
    Server
    {
        name: String,
        owner_id: Uuid,
        user_ids: Vec<Uuid>
    },
}

impl TryFrom<&ChatType> for MongolChatType
{
    type Error = ServerError;
    
    fn try_from(value: &ChatType) -> Result<Self, Self::Error> 
    {
        match value
        {
            ChatType::Private { owners } => 
            {
                let owner_ids = owners
                    .into_iter()
                    .map(|owner| mongol_helper::convert_domain_id_to_mongol(&owner.id))
                    .collect::<Result<_, _>>()?;

                Ok(
                    Self::Private 
                    { 
                        owner_ids: owner_ids
                    }
                )
            },
            ChatType::Group { name, owner, users } => 
            {
                let owner_id = mongol_helper::convert_domain_id_to_mongol(&owner.id)?;

                let user_ids = users
                    .into_iter()
                    .map(|owner| mongol_helper::convert_domain_id_to_mongol(&owner.id))
                    .collect::<Result<_, _>>()?;

                Ok(
                    Self::Group
                    {
                        name: name.to_string(),
                        owner_id: owner_id,
                        user_ids: user_ids,
                    }
                )
            },
            ChatType::Server { name, owner, users } => 
            {
                let owner_id = mongol_helper::convert_domain_id_to_mongol(&owner.id)?;

                let user_ids = users
                    .into_iter()
                    .map(|owner| mongol_helper::convert_domain_id_to_mongol(&owner.id))
                    .collect::<Result<_, _>>()?;

                Ok(
                    Self::Server
                    {
                        name: name.to_string(),
                        owner_id: owner_id,
                        user_ids: user_ids,
                    }
                )
            },
        }
    }
}