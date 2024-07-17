use bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::{db::mongoldb::mongol_helper, model::{chat::Chat, misc::ServerError}};

use super::{MongolChatInfo, MongolChatInfoWrapper};

#[derive(Debug, Serialize, Deserialize)]
pub enum MongolChat
{
    Private
    {
        id: Uuid,
        owner_ids: Vec<Uuid>,
        chat_info: MongolChatInfo 
    },
    Group
    {
        id: Uuid,
        name: String,
        owner_id: Uuid,
        user_ids: Vec<Uuid>,
        chat_info: MongolChatInfo,
    },
    Server
    {
        id: Uuid,
        name: String,
        owner_id: Uuid,
        user_ids: Vec<Uuid>,
        chat_infos: Vec<MongolChatInfo> 
    },
}

impl TryFrom<&Chat> for MongolChat
{
    type Error = ServerError;
    
    fn try_from(value: &Chat) -> Result<Self, Self::Error> 
    {
        match value
        {
            Chat::Private { id, owners, chat_info } => 
            {
                let db_id = mongol_helper::convert_domain_id_to_mongol(id)?;

                let owner_ids = owners
                    .into_iter()
                    .map(|owner| mongol_helper::convert_domain_id_to_mongol(&owner.id))
                    .collect::<Result<_, _>>()?;

                Ok(
                    Self::Private 
                    { 
                        id: db_id,
                        owner_ids,
                        chat_info: MongolChatInfo::try_from(chat_info)?,
                    }
                )
            },
            Chat::Group { id, name, owner, users, chat_info } => 
            {
                let db_id = mongol_helper::convert_domain_id_to_mongol(id)?;

                let owner_id = mongol_helper::convert_domain_id_to_mongol(&owner.id)?;

                let user_ids = users
                    .into_iter()
                    .map(|owner| mongol_helper::convert_domain_id_to_mongol(&owner.id))
                    .collect::<Result<_, _>>()?;

                Ok(
                    Self::Group
                    {
                        id: db_id,
                        name: name.to_string(),
                        owner_id: owner_id,
                        user_ids: user_ids,
                        chat_info: MongolChatInfo::try_from(chat_info)?,
                    }
                )
            },
            Chat::Server { id, name, owner, users, chat_infos } => 
            {
                let db_id = mongol_helper::convert_domain_id_to_mongol(id)?;

                let owner_id = mongol_helper::convert_domain_id_to_mongol(&owner.id)?;

                let user_ids = users
                    .into_iter()
                    .map(|owner| mongol_helper::convert_domain_id_to_mongol(&owner.id))
                    .collect::<Result<_, _>>()?;

                Ok(
                    Self::Server
                    {
                        id: db_id,
                        name: name.to_string(),
                        owner_id: owner_id,
                        user_ids: user_ids,
                        chat_infos: MongolChatInfoWrapper::try_from(chat_infos)?.0,
                    }
                )
            },
        }
    }
}