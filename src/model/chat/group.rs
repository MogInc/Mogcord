use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::{channel, error, user::User};
use super::channel::Channel;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group
{
    pub id: String,
    pub name: String,
    pub owner: User,
    pub users: Vec<User>,
    pub channel: Channel,
}

impl Group
{
    #[must_use]
    pub fn convert(id: String, name: String, owner: User, users: Vec<User>, channel: Channel) -> Self
    {
        Self
        {
            id,
            name,
            owner,
            users,
            channel,
        }
    }

    #[must_use]
    pub fn new(name: String, owner: User, users: Vec<User>, channel: Channel) -> Self
    {
        Self
        {
            id: Uuid::now_v7().to_string(),
            name,
            owner,
            users,
            channel,
        }
    }
}

impl channel::Parent for Group
{
    fn get_channel(&self, _: Option<&str>) -> Result<&Channel, error::Server> 
    {
        Ok(&self.channel)
    }

    fn can_read(&self, user_id: &str, _: Option<&str>) -> Result<bool, error::Server> 
    {
        Ok(self.owner.id == user_id || self.users.iter().any(|user| user.id == user_id))
    }

    fn can_write(&self, user_id: &str, _: Option<&str>) -> Result<bool, error::Server> 
    {
        Ok(self.owner.id == user_id || self.users.iter().any(|user| user.id == user_id))
    }
}