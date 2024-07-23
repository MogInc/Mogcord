mod private;
mod group;
mod server;
mod repository;

pub use private::*;
pub use group::*;
pub use server::*;

use bson::Bson;
use serde::{Deserialize, Serialize};

use crate::model::{channel_parent::ChannelParent, error};
use super::{helper, MongolChannel};


#[derive(Debug, Serialize, Deserialize)]
pub enum MongolChannelParent
{
    Private(MongolPrivate),
    Group(MongolGroup),
    Server(MongolServer),
}

impl TryFrom<&ChannelParent> for MongolChannelParent
{
    type Error = error::Server;
    
    fn try_from(value: &ChannelParent) -> Result<Self, Self::Error> 
    {
        match value
        {
            ChannelParent::Private(private_chat)=> Ok(Self::Private(MongolPrivate::try_from(private_chat)?)),
            ChannelParent::Group(group) => 
            {
                let db_id = helper::convert_domain_id_to_mongol(&group.id)?;

                let owner_id = helper::convert_domain_id_to_mongol(&group.owner.id)?;

                let user_ids = group.users
                    .iter()
                    .map(|(key, _)| helper::convert_domain_id_to_mongol(key))
                    .collect::<Result<_, _>>()?;


                let chat = MongolChat::Group
                {
                    name: group.name.to_string(),
                    owner_id,
                    user_ids,
                    channel: MongolChannel::try_from(&group.channel)?,
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

impl From<ChannelParent> for Bson 
{
    fn from(chat: ChannelParent) -> Bson 
    {
        Bson::String(chat.to_string())
    }
}