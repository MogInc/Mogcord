mod repository;


use std::collections::HashSet;
use mongodb::bson::Uuid;
use serde::{Serialize, Deserialize};

use crate::model::channel::Role;
use crate::model::channel_parent::Server;
use crate::model::{channel::Channel, error};
use crate::db::mongol::helper;


#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::pub_underscore_fields)]
#[allow(clippy::used_underscore_binding)]
pub struct MongolChannel
{
    pub _id : Uuid,
    pub name: Option<String>,
    pub roles: HashSet<Role>
}

impl TryFrom<&Channel> for MongolChannel
{
    type Error = error::Server;

    fn try_from(value: &Channel) -> Result<Self, Self::Error>
    {
        let chat_id = helper::convert_domain_id_to_mongol(&value.id)?;

        Ok(
            Self 
            {
                _id: chat_id,
                name: value.name.clone(),
                roles: value.roles.clone(),
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
            .map(MongolChannel::try_from)
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
            value.roles.clone(),
        )
    }
}