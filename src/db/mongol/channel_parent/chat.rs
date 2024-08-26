mod group;
mod private;

pub use group::*;
pub use private::*;

use serde::{Deserialize, Serialize};

use crate::bubble;
use crate::model::channel_parent::chat::Chat;
use crate::model::error;

#[derive(Debug, Serialize, Deserialize)]
pub enum MongolChat
{
    Private(MongolPrivate),
    Group(MongolGroup),
}

impl TryFrom<&Chat> for MongolChat
{
    type Error = error::Server<'static>;

    fn try_from(value: &Chat) -> Result<Self, Self::Error>
    {
        match value
        {
            Chat::Private(private) => Ok(Self::Private(bubble!(MongolPrivate::try_from(private))?)),
            Chat::Group(group) => Ok(Self::Group(bubble!(MongolGroup::try_from(group))?)),
        }
    }
}
