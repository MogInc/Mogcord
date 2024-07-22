mod role;
mod repository;

use std::collections::HashMap;

pub use role::*;
pub use repository::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{channel::{self, Channel}, error, user::User};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server
{
    pub id: String,
    pub name: String,
    pub owner: User,
    pub users: HashMap<String, User>,
    pub channels: Vec<Channel>,
    pub roles: HashMap<User, Vec<Role>>,
}

impl Server
{
    #[must_use]
    pub fn convert(
        id: String, 
        name: String, 
        owner: User, 
        users: HashMap<String, User>, 
        channels: Vec<Channel>, 
        roles: HashMap<User, Vec<Role>>
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
            channels: vec![channel],
            roles: HashMap::new(),
        };

        server.is_server_meeting_requirements()?;

        Ok(server)
    }
}


impl Server
{
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