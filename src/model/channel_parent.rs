mod repository;
mod rights;
mod roles;
pub mod server;
pub mod chat;

use chat::Chat;
pub use repository::*;
pub use rights::*;
pub use roles::*;
pub use server::Server;

use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::model::user::User;
use super::{channel::{Channel, Parent}, error};


#[derive(Clone, Display, Debug, Serialize, Deserialize)]
pub enum ChannelParent
{
    Chat(Chat),
    Server(Server),
}


impl ChannelParent
{
    pub fn get_channel(&self, channel_id_option: Option<&str>) -> Result<&Channel, error::Server>
    {
        match self
        {
            ChannelParent::Chat(chat) => chat.get_channel(channel_id_option),
            ChannelParent::Server(server) => server.get_channel(channel_id_option),
        }
    }

    #[must_use]
    pub fn is_chat(&self) -> bool
    {
        matches!(self, ChannelParent::Chat(_))
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
            ChannelParent::Chat(value) => value.add_user(user),
            ChannelParent::Server(value) => value.add_user(user),
        }
    }

    pub fn add_users(&mut self, users: Vec<User>) -> Result<(), error::Server>
    {
        match self
        {
            ChannelParent::Chat(value) => value.add_users(users),
            ChannelParent::Server(value) => value.add_users(users),
        }
    }

    #[must_use]
    pub fn is_owner(&self, user_id: &str) -> bool
    {
        match self
        {
            ChannelParent::Chat(value) => value.is_owner(user_id),
            ChannelParent::Server(value) => value.is_owner(user_id),
        }
    }

    #[must_use]
    pub fn is_user_part_of_chat(&self, other_user_id: &str) -> bool
    {
        match self
        {
            ChannelParent::Chat(value) => value.is_user_part_of_chat(other_user_id),
            ChannelParent::Server(value) => value.is_user_part_of_server(other_user_id),
        }
    }
}