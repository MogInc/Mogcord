use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{chat::Info, error, user::User};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server
{
    pub id: String,
    pub name: String,
    pub owner: User,
    pub users: Vec<User>,
    pub chat_infos: Vec<Info>,
}

impl Server
{
    #[must_use]
    pub fn convert(id: String, name: String, owner: User, users: Vec<User>, chat_infos: Vec<Info>) -> Self
    {
        Self
        {
            id,
            name,
            owner,
            users,
            chat_infos,
        }
    }

    #[must_use]
    pub fn new(name: String, owner: User) -> Result<Self, error::Server>
    {
        let chat_info = Info::new(Some(String::from("Welcome")));


        let server = Self
        {
            id: Uuid::now_v7().to_string(),
            name,
            owner,
            users: Vec::new(),
            chat_infos: vec![chat_info],
        };

        server.is_server_meeting_requirements()?;

        Ok(server)
    }
}


impl Server
{
    #[must_use]
    pub fn chat_infos(self) -> Vec<Info>
    {
        self.chat_infos
    }

    pub fn add_user(&mut self, user: User) -> Result<(), error::Server>
    {
        if self.is_user_part_of_server(&user.id)
        {
            return Err(error::Server::ChatAlreadyHasThisUser);
        }

        self.users.push(user);

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

        self.users.extend(users);
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
    pub fn is_user_part_of_server(&self, other_user_id: &str) -> bool
    {
        self.users.iter().any(|user| user.id == other_user_id)
    }
}