mod private;
mod group;
mod server;
mod repository;

pub use private::*;
pub use group::*;
pub use server::*;

use bson::Bson;
use serde::{Deserialize, Serialize};

use crate::model::{channel_parent::{chat::Chat, ChannelParent}, error};
use super::helper;


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
            ChannelParent::Chat(chat) => match chat 
            {
                Chat::Private(private) => Ok(Self::Private(MongolPrivate::try_from(private)?)),
                Chat::Group(group) => Ok(Self::Group(MongolGroup::try_from(group)?)),
            }
            ChannelParent::Server(server) => Ok(Self::Server(MongolServer::try_from(server)?)),
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