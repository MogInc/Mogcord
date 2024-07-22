use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::{channel, error, user::User};
use super::channel::Channel;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Private
{
    pub id: String,
    pub owners: Vec<User>,
    pub channel: Channel,
}

impl Private
{
    #[must_use]
    pub fn convert(id: String, owners: Vec<User>, channel: Channel) -> Self
    {
        Self
        {
            id,
            owners,
            channel,
        }
    }

    #[must_use]
    pub fn new(owners: Vec<User>, channel: Channel) -> Self
    {
        Self
        {
            id: Uuid::now_v7().to_string(),
            owners,
            channel,
        }
    }
}

impl channel::Parent for Private
{
    fn get_channel(&self, _: Option<&str>) -> Result<&Channel, error::Server> 
    {
        Ok(&self.channel)
    }

    fn can_read(&self, user_id: &str, _: Option<&str>) -> Result<bool, error::Server> 
    {
        Ok(self.owners.iter().any(|user| user.id == user_id))
    }

    fn can_write(&self, user_id: &str, _: Option<&str>) -> Result<bool, error::Server> 
    {
        Ok(self.owners.iter().any(|user| user.id == user_id))
    }
}