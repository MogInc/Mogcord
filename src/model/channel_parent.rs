mod repository;
mod rights;
mod role;
pub mod server;
pub mod chat;

use chat::Chat;
pub use repository::*;
pub use rights::*;
pub use role::*;
pub use server::Server;

use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::model::user::User;
use super::{channel::{self, Channel}, error};


#[derive(Clone, Display, Debug, Serialize, Deserialize)]
pub enum ChannelParent
{
    Chat(Chat),
    Server(Box<Server>),
}


impl ChannelParent
{
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

impl channel::Parent for ChannelParent
{
    fn get_channel<'input, 'stack>(
        &'input self, 
        channel_id_option: Option<&'input str>
    ) -> Result<&'input Channel, error::Server<'stack>> 
    {
        match self
        {
            ChannelParent::Chat(val) => val.get_channel(channel_id_option),
            ChannelParent::Server(val) => val.get_channel(channel_id_option),
        }
    }

    fn get_user_roles(&self, user_id: &str) -> Option<&Vec<String>> 
    {
        match self
        {
            ChannelParent::Chat(val) => val.get_user_roles(user_id),
            ChannelParent::Server(val) => val.get_user_roles(user_id),
        }
    }

    fn can_read<'input, 'stack>(
        &'input self,
        user_id: &'input str, 
        channel_id_option: Option<&'input str>
    ) -> Result<bool, error::Server<'stack>> 
    {
        match self
        {
            ChannelParent::Chat(val) => val.can_read(user_id, channel_id_option),
            ChannelParent::Server(val) => val.can_read(user_id, channel_id_option),
        }
    }

    fn can_write<'input, 'stack>(
        &'input self,
        user_id: &'input str, 
        channel_id_option: Option<&'input str>
    ) -> Result<bool, error::Server<'stack>> 
    {
        match self
        {
            ChannelParent::Chat(val) => val.can_write(user_id, channel_id_option),
            ChannelParent::Server(val) => val.can_write(user_id, channel_id_option),
        }
    }
}