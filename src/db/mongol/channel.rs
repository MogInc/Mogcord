mod repository;

use std::collections::HashSet;

use mongodb::bson::Uuid;
use serde::{Serialize, Deserialize};

use crate::model::channel::Role;
use crate::model::{channel::Channel, error};
use crate::db::mongol::helper;


#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::pub_underscore_fields)]
#[allow(clippy::used_underscore_binding)]
pub struct MongolChannel
{
    pub _id : Uuid,
    pub name: Option<String>,
    pub roles: Option<HashSet<Role>>
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

pub struct MongolChannelWrapper(pub Vec<MongolChannel>);

impl TryFrom<&Vec<Channel>> for MongolChannelWrapper
{
    type Error = error::Server;

    fn try_from(value: &Vec<Channel>) -> Result<Self, Self::Error>
    {
        let mut channel_vec = Vec::new();

        for channel in value
        {
            channel_vec.push(MongolChannel::try_from(channel)?);
        }

        Ok(MongolChannelWrapper(channel_vec))
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