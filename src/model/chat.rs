mod repository;
mod private;
mod group;

pub use repository::*;
pub use private::*;
pub use group::*;

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
        Private::owner_size()
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
            Chat::Private(private) => private.is_owner(user_id),
            Chat::Group(group) => group.is_owner(user_id),
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
}