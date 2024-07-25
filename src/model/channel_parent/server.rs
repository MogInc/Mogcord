mod repository;

pub use repository::*;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::{channel::{self, Channel, Parent}, error, user::User, ROLE_NAME_EVERYBODY};
use super::Role;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server
{
    pub id: String,
    pub name: String,
    pub owner: User,
    pub users: HashMap<String, User>,
    pub channels: HashMap<String, Channel>,
    pub roles: HashMap<String, Role>,
    pub user_roles: HashMap<String, Vec<String>>,
}

impl Server
{
    #[must_use]
    fn convert(
        id: String, 
        name: String, 
        owner: User, 
        users: HashMap<String, User>, 
        channels: HashMap<String, Channel>,
        roles: HashMap<String, Role>,
        user_roles: HashMap<String, Vec<String>>,
    ) -> Self
    {
        Self
        {
            id,
            name,
            owner,
            users,
            channels,
            roles,
            user_roles,
        }
    }

    pub fn new(name: String, owner: User) -> Result<Self, error::Server>
    {
        let base_channel = Channel::new(Some(String::from("Welcome")), true);

        let base_role = Role::new(ROLE_NAME_EVERYBODY.to_string(), 1);

        let server = Self
        {
            id: Uuid::now_v7().to_string(),
            name,
            owner,
            users: HashMap::new(),
            channels: HashMap::from([(base_channel.id.clone(), base_channel)]),
            roles: HashMap::from([(ROLE_NAME_EVERYBODY.to_string(), base_role)]),
            user_roles: HashMap::new(),
        };

        server.is_server_meeting_requirements()?;

        Ok(server)
    }
}


impl Server
{
    pub fn add_user(&mut self, user: User) -> Result<(), error::Server>
    {
        if self.is_user_part_of_server(&user.id) 
        {
            return Err(error::Server::ServerAlreadyHasThisUser);
        }

        self.users.insert(user.id.to_string(), user);

        Ok(())
    }

    pub fn add_users(&mut self, users: Vec<User>) -> Result<(), error::Server>
    {
        for user in &users 
        {
            if self.is_user_part_of_server(&user.id) 
            {
                return Err(error::Server::ServerAlreadyHasThisUser);
            }
        }

        self.users.extend(users.into_iter().map(|user| (user.id.to_string(), user)));

        Ok(())
    }

    pub fn is_server_meeting_requirements(&self) -> Result<(), error::Server> 
    {
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

    #[must_use]
    pub fn filter_channels(self, user_id: &str) -> Self
    {
        let filtered_channels = self
            .channels
            .clone()
            .into_iter()
            .filter(|(channel_id, _)| self.can_read(user_id, Some(channel_id)).unwrap_or(false))
            .collect();

        Self::convert(
            self.id, 
            self.name, 
            self.owner, 
            self.users,
            filtered_channels,
            self.roles,
            self.user_roles
        )
    }
}


impl channel::Parent for Server
{   
    fn get_channel(&self, channel_id_option: Option<&str>) -> Result<&Channel, error::Server>
    {
        match channel_id_option 
        {
            Some(id) => self.channels.get(id).ok_or(error::Server::ChannelNotFound),
            None => Err(error::Server::ChannelNotFound),
        }
    }

    fn get_user_roles(&self, user_id: &str) -> Option<&Vec<String>> 
    {
        self.user_roles.get(user_id)
    }

    fn can_read(&self, user_id: &str, channel_id_option: Option<&str>) -> Result<bool, error::Server> 
    {
        self.internal_check_permission(user_id, channel_id_option, Channel::can_role_read)
    }
    
    fn can_write(&self, user_id: &str, channel_id_option: Option<&str>) -> Result<bool, error::Server> 
    {
        self.internal_check_permission(user_id, channel_id_option, Channel::can_role_write)
    }
}

impl Server
{
    fn internal_check_permission(
        &self,
        user_id: &str,
        channel_id_option: Option<&str>,
        access_check: impl Fn(&Channel, &str) -> bool,
    ) -> Result<bool, error::Server> 
    {

        if self.is_owner(user_id) 
        {
            return Ok(true);
        }
    
        if !self.users.contains_key(user_id) 
        {
            return Ok(false);
        }
        
        let user_roles_option = self.get_user_roles(user_id);
    
        let channel_id = channel_id_option
            .ok_or(error::Server::ChannelNotPassed)?;
    
        let channel = self
            .channels
            .get(channel_id)
            .ok_or(error::Server::ChannelNotFound)?;
    
        let roles_default: &Vec<String> = &Vec::new();
        let user_roles: &Vec<String> = user_roles_option.unwrap_or(roles_default);
    
        for user_role in user_roles
        {
            if access_check(channel, user_role) 
            {
                return Ok(true);
            }
        }
    
        Ok(false)
    }
}