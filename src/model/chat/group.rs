
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::model::{channel, error, user::User};
use super::channel::Channel;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group
{
    pub id: String,
    pub name: String,
    pub owner: User,
    pub users: HashMap<String, User>,
    pub channel: Channel,
}

impl Group
{
    #[must_use]
    pub fn convert(id: String, name: String, owner: User, users: HashMap<String, User>, channel: Channel) -> Self
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

    pub fn new(name: String, owner: User, users: Vec<User>) -> Result<Self, error::Server> 
    {
        let users_sanitized = users
            .into_iter()
            .filter(|user| user.id != owner.id)
            .map(|user| Ok((user.id.clone(), user)))
            .collect::<Result<_, _>>()?;

        let channel = Channel::new(None);

        let group = Group::convert(channel.id.to_string(), name, owner, users_sanitized, channel);

        group.internal_is_meeting_requirements()?;

        Ok(group)
    }
}

impl Group
{
    #[must_use]
    pub fn is_owner(&self, user_id: &str) -> bool
    {
        self.owner.id == user_id
    }

    pub fn is_user_part_of_server(&self, other_user: &str) -> bool
    {
        self.is_owner(other_user) || self.users.contains_key(other_user)
    }

    fn internal_is_meeting_requirements(&self) -> Result<(), error::Server> 
    {
        Ok(())
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
        Ok(self.is_user_part_of_server(user_id))
    }

    fn can_write(&self, user_id: &str, _: Option<&str>) -> Result<bool, error::Server> 
    {
        Ok(self.is_user_part_of_server(user_id))
    }
}