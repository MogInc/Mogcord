use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::{channel::{self, Channel}, error, user::User, ROLE_NAME_EVERYBODY};
use super::Roles;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server
{
    pub id: String,
    pub name: String,
    pub owner: User,
    pub users: HashMap<String, User>,
    pub channels: HashMap<String, Channel>,
    pub roles: HashMap<User, Roles>,
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
        roles: HashMap<User, Roles>
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
        }
    }

    pub fn new(name: String, owner: User) -> Result<Self, error::Server>
    {
        let channel = Channel::new(Some(String::from("Welcome")));


        let server = Self
        {
            id: Uuid::now_v7().to_string(),
            name,
            owner,
            users: HashMap::new(),
            channels: HashMap::from([(channel.id.clone(), channel)]),
            roles: HashMap::new(),
        };

        server.is_server_meeting_requirements()?;

        Ok(server)
    }
}


impl Server
{
    pub fn channels(&self) -> &Vec<Channel>
    {
        self.channels().into_values().collect()
    }

    pub fn users(&self) -> &Vec<User>
    {
        self.users.into_values().collect()
    }

    pub fn add_user(&mut self, user: User) -> Result<(), error::Server>
    {
        let insert_option = self.users.insert(user.id.to_string(), user);

        if insert_option.is_none()
        {
            return Err(error::Server::ChatAlreadyHasThisUser);
        }


        Ok(())
    }

    pub fn add_users(&mut self, users: Vec<User>) -> Result<(), error::Server>
    {
        for user in &users 
        {
            if self.is_user_part_of_server(&user.id) 
            {
                return Err(error::Server::ChatAlreadyHasThisUser);
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

    fn can_read(&self, user_id: &str, channel_id_option: Option<&str>) -> Result<bool, error::Server> 
    {
        self.check_permission(user_id, channel_id_option, channel::Role::can_read)
    }
    
    fn can_write(&self, user_id: &str, channel_id_option: Option<&str>) -> Result<bool, error::Server> 
    {
        self.check_permission(user_id, channel_id_option, channel::Role::can_write)
    }
}

impl Server
{
    fn check_permission<F>(
        &self, 
        user_id: &str, 
        channel_id_option: 
        Option<&str>, 
        check_role: F
    ) -> Result<bool, error::Server>
    where
        F: Fn(&channel::Role) -> Option<bool>,
    {
        if self.is_owner(user_id) 
        {
            return Ok(true);
        }
    
        if !self.users.contains_key(user_id) 
        {
            return Ok(false);
        }
    
        let user = self
            .users
            .get(user_id)
            .ok_or(error::Server::UserNotFound)?;
    
        let user_roles_option = self.roles.get(user);
    
        let channel_id = channel_id_option
            .ok_or(error::Server::ChannelNotPassed)?;
    
        let channel = self
            .channels
            .get(channel_id)
            .ok_or(error::Server::ChannelNotFound)?;
    
        let channel_roles = &channel.roles;
        let roles_default = &Roles::default();
        let user_roles = user_roles_option.unwrap_or(roles_default);
    
        for role in channel_roles 
        {
            if role.name == ROLE_NAME_EVERYBODY 
            {
                return Ok(check_role(role).unwrap_or(true));
            }
    
            if !user_roles.contains(&role.name) 
            {
                continue;
            }
    
            if let Some(b) = check_role(role) 
            {
                return Ok(b);
            }
        }
    
        Ok(false)
    }
}