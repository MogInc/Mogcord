mod repository;
mod private;
mod group;
mod server;
mod rights;
mod roles;

pub use repository::*;
pub use private::*;
pub use group::*;
pub use server::*;
pub use rights::*;
pub use roles::*;

use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::model::user::User;
use super::{channel::{Channel, Parent}, error};


#[derive(Clone, Display, Debug, Serialize, Deserialize)]
pub enum ChannelParent
{
    Private(Private),
    Group(Group),
    Server(Server),
}


impl ChannelParent
{
    #[must_use]
    pub fn get_channel(&self, channel_id_option: Option<&str>) -> Result<&Channel, error::Server>
    {
        match self
        {
            ChannelParent::Private(private) => private.get_channel(channel_id_option),
            ChannelParent::Group(group) => group.get_channel(channel_id_option),
            ChannelParent::Server(server) => server.get_channel(channel_id_option),
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
        matches!(self, ChannelParent::Private(_))
    }

    #[must_use]
    pub fn is_group(&self) -> bool
    {
        matches!(self, ChannelParent::Group(_))
    }

    #[must_use]
    pub fn is_server(&self) -> bool
    {
        matches!(self, ChannelParent::Server(_))
    }

    pub fn add_user(&mut self, user: User) -> Result<(), error::Server>
    {
        match self
        {
            ChannelParent::Private(_) => Err(error::Server::ChatNotAllowedToGainUsers),
            ChannelParent::Group(group) => group.add_user(user),
            ChannelParent::Server(server) => server.add_user(user),
        }
    }

    pub fn add_users(&mut self, users: Vec<User>) -> Result<(), error::Server>
    {
        match self
        {
            ChannelParent::Private(_) => Err(error::Server::ChatNotAllowedToGainUsers),
            ChannelParent::Group(group) => group.add_users(users),
            ChannelParent::Server(server) => server.add_users(users),
        }
    }

    #[must_use]
    pub fn is_owner(&self, user_id: &str) -> bool
    {
        match self
        {
            ChannelParent::Private(private) => private.is_owner(user_id),
            ChannelParent::Group(group) => group.is_owner(user_id),
            ChannelParent::Server(server) => server.is_owner(user_id),
        }
    }

    #[must_use]
    pub fn is_user_part_of_chat(&self, other_user_id: &str) -> bool
    {
        match self
        {
            ChannelParent::Private(private) => private.is_owner(other_user_id),
            ChannelParent::Group(group) => group.is_user_part_of_server(other_user_id),
            ChannelParent::Server(server) => server.is_user_part_of_server(other_user_id),
        }
    }
}