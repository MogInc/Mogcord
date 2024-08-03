
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::{model::{channel::{self, Channel}, error, user::User}, server_error};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group
{
    pub id: String,
    pub name: String,
    pub owner: User,
    //key is user id
    pub users: HashMap<String, User>,
    pub channel: Channel,
}

impl Group
{
    #[must_use]
    fn convert(id: String, name: String, owner: User, users: HashMap<String, User>, channel: Channel) -> Self
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

    pub fn new<'err>(name: String, owner: User, users: Vec<User>) -> error::Result<'err, Self> 
    {
        let users_sanitized = users
            .into_iter()
            .filter(|user| user.id != owner.id)
            .map(|user| Ok((user.id.clone(), user)))
            .collect::<Result<_, _>>()?;

        let channel = Channel::new(None, false);

        let group = Group::convert(channel.id.to_string(), name, owner, users_sanitized, channel);

        group.internal_is_meeting_requirements()?;

        Ok(group)
    }
}

impl Group
{
    const GROUP_USER_MIN: usize = 2;

    pub fn add_user<'err>(&mut self, user: User) -> error::Result<'err, ()>
    {
        if self.is_user_part_of_server(&user.id) 
        {
            return Err(server_error!(error::Kind::AlreadyPartOf, error::OnType::ChatGroup)
                .add_debug_info("user id", user.id)
            );
        }

        self.users.insert(user.id.to_string(), user);

        Ok(())
    }

    pub fn add_users<'err>(&mut self, users: Vec<User>) -> error::Result<'err, ()>
    {
        for user in &users 
        {
            if self.is_user_part_of_server(&user.id) 
            {
                return Err(server_error!(error::Kind::AlreadyPartOf, error::OnType::ChatGroup)
                    .add_debug_info("user id", user.id.to_string())
                );
            }
        }

        self.users.extend(users.into_iter().map(|user| (user.id.to_string(), user)));

        Ok(())
    }

    #[must_use]
    pub fn is_owner(&self, user_id: &str) -> bool
    {
        self.owner.id == user_id
    }

    #[must_use]
    pub fn is_user_part_of_server(&self, other_user: &str) -> bool
    {
        self.is_owner(other_user) || self.users.contains_key(other_user)
    }

    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::unused_self)]
    fn internal_is_meeting_requirements<'err>(&self) -> error::Result<'err, ()> 
    {
        if self.users.len() < Self::GROUP_USER_MIN
        {
            return Err(server_error!(error::Kind::InValid, error::OnType::User)
                .add_public_info(format!("Expected atleast: {}, found: {}", Self::GROUP_USER_MIN, self.users.len()))
            );
        }

        Ok(())
    }
}

impl channel::Parent for Group
{
    fn get_channel<'input, 'err>(
        &'input self, 
        _: Option<&'input str>
    ) -> error::Result<'err, &'input Channel> 
    {
        Ok(&self.channel)
    }

    fn get_user_roles(&self, _: &str) -> Option<&Vec<String>> 
    {
        None
    }

    fn can_read<'input, 'err>(
        &'input self, 
        user_id: &'input str, 
        _: Option<&'input str>
    ) -> error::Result<'err, bool> 
    {
        Ok(self.is_user_part_of_server(user_id))
    }

    fn can_write<'input, 'err>(
        &'input self, 
        user_id: &'input str, 
        _: Option<&'input str>
    ) -> error::Result<'err, bool> 
    {
        Ok(self.is_user_part_of_server(user_id))
    }
}