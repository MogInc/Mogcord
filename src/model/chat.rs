mod repository;
mod private;
mod group;

pub use repository::*;
pub use private::*;
pub use group::*;

use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::model::user::User;
use super::{channel::{self, Channel}, error};


#[derive(Clone, Display, Debug, Serialize, Deserialize)]
pub enum Chat
{
    Private(Private),
    Group(Group),
}


impl Chat
{
    pub fn new_private(owners: Vec<User>) -> Result<Self, error::Server> 
    {
        let set: HashSet<User> = owners
            .into_iter()
            .collect();

        let owners_sanitized: Vec<User> = set
            .into_iter()
            .collect();

        let channel = Channel::new(None);

        let private_chat = Private::convert(channel.id.to_string(), owners_sanitized, channel);

        let chat_type = Chat::Private(private_chat);

        chat_type.is_chat_meeting_requirements()?;

        Ok(chat_type)
    }

    pub fn new_group(name: String, owner: User, users: Vec<User>) -> Result<Self, error::Server> 
    {
        let users_sanitized: Vec<User> = users
            .into_iter()
            .filter(|user| user.id != owner.id)
            .collect();

        let channel = Channel::new(None);

        let group_chat = Group::convert(channel.id.to_string(), name, owner, users_sanitized, channel);

        let chat_type = Chat::Group(group_chat);

        chat_type.is_chat_meeting_requirements()?;

        Ok(chat_type)
    }
}

impl Chat
{
    const PRIVATE_OWNER_MAX: usize = 2;
    const GROUP_OWNER_MAX: usize = 1;

    #[must_use]
    pub fn channel(&self) -> Channel
    {
        match self
        {
            Chat::Private(private) => private.channel.clone(),
            Chat::Group(group) => group.channel.clone(),
        }
    }

    #[must_use]
    pub fn private_owner_size() -> usize
    {
        Self::PRIVATE_OWNER_MAX
    }

    #[must_use]
    pub fn is_private(&self) -> bool
    {
        matches!(self, Chat::Private(_))
    }

    #[must_use]
    pub fn is_group(&self) -> bool
    {
        matches!(self, Chat::Group(_))
    }

    pub fn add_user(&mut self, user: User) -> Result<(), error::Server>
    {
        if self.is_user_part_of_chat(&user.id)
        {
            return Err(error::Server::ChatAlreadyHasThisUser);
        }

        match self
        {
            Chat::Private(_) => Err(error::Server::ChatNotAllowedToGainUsers),
            Chat::Group(group) => 
            {
                group.users.push(user);
                Ok(())
            },
        }
    }

    pub fn add_users(&mut self, users: Vec<User>) -> Result<(), error::Server>
    {
        for user in &users 
        {
            if self.is_user_part_of_chat(&user.id) 
            {
                return Err(error::Server::ChatAlreadyHasThisUser);
            }
        }

        match self
        {
            Chat::Private(_) => Err(error::Server::ChatNotAllowedToGainUsers),
            Chat::Group(group) => 
            {
                group.users.extend(users);
                Ok(())
            },
        }
    }

    #[must_use]
    pub fn is_owner(&self, user_id: &str) -> bool
    {
        match self
        {
            Chat::Private(private) => private.owners.iter().any(|user| user.id == user_id),
            Chat::Group(group) => group.owner.id == user_id,
        }
    }

    pub fn is_chat_meeting_requirements(&self) -> Result<(), error::Server> 
    {
        match self
        {
            Chat::Private(private) => 
            {
                let user_len = private.owners.len();

                if !self.internal_is_owner_size_allowed(user_len)
                {
                    return Err(error::Server::OwnerCountInvalid { expected: Self::PRIVATE_OWNER_MAX, found: user_len });
                }

                Ok(())
            },
            Chat::Group(_) => Ok(()),
        }
    }

    #[must_use]
    pub fn is_user_part_of_chat(&self, other_user_id: &str) -> bool
    {
        match self
        {
            Chat::Private(private) => private.owners.iter().any(|owner| owner.id == other_user_id),
            Chat::Group(group) => group.owner.id == other_user_id
                || group.users.iter().any(|user| user.id == other_user_id),
        }
    }

    fn internal_is_owner_size_allowed(&self, owner_count: usize) -> bool
    {
        match self
        {
            Chat::Private(_) => owner_count == Self::PRIVATE_OWNER_MAX,
            Chat::Group(_) => owner_count == Self::GROUP_OWNER_MAX,
        }
    }
}