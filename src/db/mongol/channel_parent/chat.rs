mod group;
mod private;

use bson::Document;
pub use group::*;
pub use private::*;

use serde::{Deserialize, Serialize};

use crate::model::{channel_parent::chat::Chat, error};

#[derive(Debug, Serialize, Deserialize)]
pub enum MongolChat
{
    Private(MongolPrivate),
    Group(MongolGroup),
}

impl TryFrom<&Chat> for MongolChat
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