mod private;
mod group;
mod server;
mod repository;

pub use private::*;
pub use group::*;
pub use server::*;

use bson::Bson;
use serde::{Deserialize, Serialize};

use crate::model::{channel_parent::{chat::Chat, ChannelParent, Server}, error};
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
            ChannelParent::Chat(chat) => MongolChannelParent::try_from(chat),
            ChannelParent::Server(server) => MongolChannelParent::try_from(server),
        }
    }
}

impl TryFrom<&Chat> for MongolChannelParent
{
    type Error = error::Server;
    
    fn try_from(value: &Chat) -> Result<Self, Self::Error> 
    {
        match value 
        {
            Chat::Private(private) => Ok(Self::Private(MongolPrivate::try_from(private)?)),
            Chat::Group(group) => Ok(Self::Group(MongolGroup::try_from(group)?)),
        }
    }
}

impl TryFrom<&Server> for MongolChannelParent
{
    type Error = error::Server;
    
    fn try_from(value: &Server) -> Result<Self, Self::Error> 
    {
        Ok(Self::Server(MongolServer::try_from(value)?))
    }
}


impl From<ChannelParent> for Bson 
{
    fn from(chat: ChannelParent) -> Bson 
    {
        Bson::String(chat.to_string())
    }
}