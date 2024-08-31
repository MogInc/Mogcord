use serde::Serialize;

use crate::model::channel::Channel;

use super::ObjectToDTO;

#[derive(Serialize)]
pub struct ChannelCreateResponse
{
    pub id: String,
    pub name: Option<String>,
}

impl ObjectToDTO<Channel> for ChannelCreateResponse
{
    fn obj_to_dto(channel: Channel) -> Self
    {
        Self {
            id: channel.id,
            name: channel.name,
        }
    }
}

#[derive(Serialize)]
pub struct ChannelGetResponse
{
    pub id: String,
    pub name: Option<String>,
}

impl ObjectToDTO<Channel> for ChannelGetResponse
{
    fn obj_to_dto(channel: Channel) -> Self
    {
        Self {
            id: channel.id,
            name: channel.name,
        }
    }
}
