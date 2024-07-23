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
            ChannelParent::Private(private)=> Ok(Self::Private(MongolPrivate::try_from(private)?)),
            ChannelParent::Group(group) => Ok(Self::Group(MongolGroup::try_from(group)?)),
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