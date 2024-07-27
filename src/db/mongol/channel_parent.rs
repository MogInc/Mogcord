mod chat;
mod server;
mod repository;

pub use chat::*;
pub use server::*;

use bson::Bson;
use serde::{Deserialize, Serialize};

use crate::model::{channel_parent::{chat::Chat, ChannelParent, Server}, error};
use super::helper;


#[derive(Debug, Serialize, Deserialize)]
pub enum MongolChannelParent
{
    Chat(MongolChat),
    Server(MongolServer),
}

impl TryFrom<&ChannelParent> for MongolChannelParent
{
    type Error = error::Server<'static>;
    
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
    type Error = error::Server<'static>;
    
    fn try_from(value: &Chat) -> Result<Self, Self::Error> 
    {
        Ok(Self::Chat(MongolChat::try_from(value)?))
    }
}

impl TryFrom<&Box<Server>> for MongolChannelParent
{
    type Error = error::Server<'static>;
    
    fn try_from(value: &Box<Server>) -> Result<Self, Self::Error> 
    {
        Ok(Self::Server(MongolServer::try_from(&**value)?))
    }
}


impl From<ChannelParent> for Bson 
{
    fn from(chat: ChannelParent) -> Bson 
    {
        Bson::String(chat.to_string())
    }
}