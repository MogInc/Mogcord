mod repository;
mod group;
mod private;

pub use repository::*;
pub use group::*;
pub use private::*;

use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::model::{channel::{self, Channel}, error, user::User};

#[derive(Clone, Display, Debug, Serialize, Deserialize)]
pub enum Chat
{
    Private(Private),
    Group(Group),
}

impl Chat
{
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
        match self
        {
            Chat::Private(_) => Err(error::Server::ChatNotAllowedToGainUsers),
            Chat::Group(group) => group.add_user(user),
        }
    }

    pub fn add_users(&mut self, users: Vec<User>) -> Result<(), error::Server>
    {
        match self
        {
            Chat::Private(_) => Err(error::Server::ChatNotAllowedToGainUsers),
            Chat::Group(group) => group.add_users(users),
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
            Chat::Private(private) => private.is_owner(other_user_id),
            Chat::Group(group) => group.is_user_part_of_server(other_user_id),
        }
    }
}

impl channel::Parent for Chat
{
    fn get_channel(&self, channel_id_option: Option<&str>) -> Result<&Channel, error::Server>
    {
        match self
        {
            Chat::Private(private) => private.get_channel(channel_id_option),
            Chat::Group(group) => group.get_channel(channel_id_option),
        }
    }

    fn can_read(&self, user_id: &str, channel_id_option: Option<&str>) -> Result<bool, error::Server> 
    {
        match self
        {
            Chat::Private(val) => val.can_read(user_id, channel_id_option),
            Chat::Group(val) => val.can_read(user_id, channel_id_option),
        }
    }

    fn can_write(&self, user_id: &str, channel_id_option: Option<&str>) -> Result<bool, error::Server> 
    {
        match self
        {
            Chat::Private(val) => val.can_write(user_id, channel_id_option),
            Chat::Group(val) => val.can_write(user_id, channel_id_option),
        }
    }
}