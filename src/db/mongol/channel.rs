mod repository;


use mongodb::bson::Uuid;
use serde::{Serialize, Deserialize};

use crate::model::channel::Role;
use crate::model::channel_parent::chat::Chat;
use crate::model::channel_parent::Server;
use crate::model::{channel::Channel, error};
use crate::db::mongol::helper;


#[derive(Debug, Serialize, Deserialize)]
pub enum ParentType
{
    ChatPrivate,
    ChatGroup,
    Server,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::pub_underscore_fields)]
#[allow(clippy::used_underscore_binding)]
pub struct MongolChannel
{
    pub _id : Uuid,
    pub parent_type: ParentType,
    pub name: Option<String>,
    pub roles: Vec<Role>
}

impl TryFrom<&Chat> for MongolChannel
{
    type Error = error::Server;

    fn try_from(value: &Chat) -> Result<Self, Self::Error>
    {
        let mongol_channel = match value
        {
            Chat::Private(val) => MongolChannel::try_from((&val.channel, ParentType::ChatPrivate))?,
            Chat::Group(val) => MongolChannel::try_from((&val.channel, ParentType::ChatGroup))?,
        };

        Ok(mongol_channel)
    }
}

impl TryFrom<(&Channel, ParentType)> for MongolChannel
{
    type Error = error::Server;

    fn try_from((value, parent_type): (&Channel, ParentType)) -> Result<Self, Self::Error>
    {
        let channel_id = helper::convert_domain_id_to_mongol(&value.id)?;

        Ok(
            Self 
            {
                _id: channel_id,
                parent_type,
                name: value.name.clone(),
                roles: value.roles.iter().cloned().collect(),
            }
        )
    }
}

pub struct MongolChannelVecWrapper(pub Vec<MongolChannel>);

impl TryFrom<&Server> for MongolChannelVecWrapper
{
    type Error = error::Server;

    fn try_from(value: &Server) -> Result<Self, Self::Error>
    {
        let mongol_channels = value
            .channels
            .values()
            .map(|channel| MongolChannel::try_from((channel, ParentType::Server)))
            .collect::<Result<_,_>>()?;

        Ok(Self(mongol_channels))
    }
}

impl From<&MongolChannel> for Channel
{
    fn from(value: &MongolChannel) -> Self 
    {
        Channel::convert(
            value._id.to_string(),
            value.name.clone(),
            value.roles.iter().cloned().collect(),
        )
    }
}