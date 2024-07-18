use bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::{db::mongoldb::mongol_helper, model::{chat::Chat, misc::ServerError}};

use super::{MongolChatInfo, MongolChatInfoWrapper};

//reason for wrapper
//else _id gets an ObjectId signed and will most likely do some voodoo to retrieve a chat
#[derive(Debug, Serialize, Deserialize)]
pub struct MongolChatWrapper
{
    pub _id: Uuid,
    pub chat: MongolChat,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MongolChat
{
    Private
    {
        owner_ids: Vec<Uuid>,
        chat_info: MongolChatInfo 
    },
    Group
    {
        name: String,
        owner_id: Uuid,
        user_ids: Vec<Uuid>,
        chat_info: MongolChatInfo,
    },
    Server
    {
        name: String,
        owner_id: Uuid,
        user_ids: Vec<Uuid>,
        chat_infos: Vec<MongolChatInfo> 
    },
}

impl TryFrom<&Chat> for MongolChatWrapper
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

                let chat = MongolChat::Private 
                { 
                    owner_ids,
                    chat_info: MongolChatInfo::try_from(chat_info)?,
                };

                Ok(
                    Self 
                    { 
                        _id: db_id,
                        chat
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


                let chat = MongolChat::Group
                {
                    name: name.to_string(),
                    owner_id: owner_id,
                    user_ids: user_ids,
                    chat_info: MongolChatInfo::try_from(chat_info)?,
                };

                Ok(
                    Self 
                    { 
                        _id: db_id,
                        chat
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


                let chat = MongolChat::Server
                { 
                    name: name.to_string(),
                    owner_id: owner_id,
                    user_ids: user_ids,
                    chat_infos: MongolChatInfoWrapper::try_from(chat_infos)?.0,
                };

                Ok(
                    Self 
                    { 
                        _id: db_id,
                        chat
                    }
                )
            },
        }
    }
}