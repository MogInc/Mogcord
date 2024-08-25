mod group;
mod private;
mod repository;

pub use group::*;
pub use private::*;
pub use repository::*;

use serde::{
    Deserialize,
    Serialize,
};
use strum_macros::Display;

use crate::model::channel::{
    self,
    Channel,
};
use crate::model::error::{
    self,
    Kind,
    OnType,
};
use crate::model::user::User;
use crate::server_error;

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

    pub fn add_user<'err>(
        &mut self,
        user: User,
    ) -> error::Result<'err, ()>
    {
        match self
        {
            Chat::Private(_) => Err(server_error!(
                Kind::CantGainUsers,
                OnType::ChatPrivate
            )),
            Chat::Group(group) => group.add_user(user),
        }
    }

    pub fn add_users<'err>(
        &mut self,
        users: Vec<User>,
    ) -> error::Result<'err, ()>
    {
        match self
        {
            Chat::Private(_) => Err(server_error!(
                Kind::CantGainUsers,
                OnType::ChatPrivate
            )),
            Chat::Group(group) => group.add_users(users),
        }
    }

    #[must_use]
    pub fn is_owner(
        &self,
        user_id: &str,
    ) -> bool
    {
        match self
        {
            Chat::Private(private) => private.is_owner(user_id),
            Chat::Group(group) => group.is_owner(user_id),
        }
    }

    #[must_use]
    pub fn is_user_part_of_chat(
        &self,
        other_user_id: &str,
    ) -> bool
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
    fn get_channel<'input, 'err>(
        &'input self,
        channel_id_option: Option<&'input str>,
    ) -> error::Result<'err, &'input Channel>
    {
        match self
        {
            Chat::Private(private) => private.get_channel(channel_id_option),
            Chat::Group(group) => group.get_channel(channel_id_option),
        }
    }

    fn get_user_roles(
        &self,
        user_id: &str,
    ) -> Option<&Vec<String>>
    {
        match self
        {
            Chat::Private(val) => val.get_user_roles(user_id),
            Chat::Group(val) => val.get_user_roles(user_id),
        }
    }

    fn can_read<'input, 'err>(
        &'input self,
        user_id: &'input str,
        channel_id_option: Option<&'input str>,
    ) -> error::Result<'err, bool>
    {
        match self
        {
            Chat::Private(val) => val.can_read(user_id, channel_id_option),
            Chat::Group(val) => val.can_read(user_id, channel_id_option),
        }
    }

    fn can_write<'input, 'err>(
        &'input self,
        user_id: &'input str,
        channel_id_option: Option<&'input str>,
    ) -> error::Result<'err, bool>
    {
        match self
        {
            Chat::Private(val) => val.can_write(user_id, channel_id_option),
            Chat::Group(val) => val.can_write(user_id, channel_id_option),
        }
    }
}
