mod info;
mod repository;

pub use info::*;


use bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::model::{chat::Chat, error};
use super::helper;

//reason for wrapper
//else _id gets an ObjectId signed and will most likely do some voodoo to retrieve a chat
#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::pub_underscore_fields)]
#[allow(clippy::used_underscore_binding)]
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
}

impl TryFrom<&Chat> for MongolChatWrapper
{
    type Error = error::Server;
    
    fn try_from(value: &Chat) -> Result<Self, Self::Error> 
    {
        match value
        {
            Chat::Private(private_chat)=> 
            {
                let db_id = helper::convert_domain_id_to_mongol(&private_chat.id)?;

                let owner_ids = private_chat.owners
                    .iter()
                    .map(|owner| helper::convert_domain_id_to_mongol(&owner.id))
                    .collect::<Result<_, _>>()?;

                let chat = MongolChat::Private 
                { 
                    owner_ids,
                    chat_info: MongolChatInfo::try_from(&private_chat.chat_info)?,
                };

                Ok(
                    Self 
                    { 
                        _id: db_id,
                        chat
                    }
                )
            },
            Chat::Group(group) => 
            {
                let db_id = helper::convert_domain_id_to_mongol(&group.id)?;

                let owner_id = helper::convert_domain_id_to_mongol(&group.owner.id)?;

                let user_ids = group.users
                    .iter()
                    .map(|owner| helper::convert_domain_id_to_mongol(&owner.id))
                    .collect::<Result<_, _>>()?;


                let chat = MongolChat::Group
                {
                    name: group.name.to_string(),
                    owner_id,
                    user_ids,
                    chat_info: MongolChatInfo::try_from(&group.chat_info)?,
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