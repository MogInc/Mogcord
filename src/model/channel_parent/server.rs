mod repository;

pub use repository::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::Role;
use crate::model::channel::{self, Channel, Parent};
use crate::model::user::User;
use crate::model::{error, ROLE_NAME_EVERYBODY};
use crate::server_error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server
{
    pub id: String,
    pub name: String,
    pub owner: User,
    //key is user id
    pub users: HashMap<String, User>,
    //key is channel id
    pub channels: HashMap<String, Channel>,
    //key is role name
    pub roles: HashMap<String, Role>,
    //key is user_id
    //value are role names => can become a HashSet if Vec becomes slow
    //can work with full obj but seems like waste
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
        Self {
            id,
            name,
            owner,
            users,
            channels,
            roles,
            user_roles,
        }
    }

    pub fn new<'err>(name: String, owner: User) -> error::Result<'err, Self>
    {
        let base_channel = Channel::new(Some(String::from("Welcome")), true);

        let base_role = Role::new(ROLE_NAME_EVERYBODY.to_string(), 1);

        let server = Self {
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
    pub fn add_user<'err>(&mut self, user: User) -> error::Result<'err, ()>
    {
        if self.is_user_part_of_server(&user.id)
        {
            return Err(
                server_error!(error::Kind::AlreadyPartOf, error::OnType::Server)
                    .add_debug_info("user id", user.id),
            );
        }

        self.users.insert(user.id.to_string(), user);

        Ok(())
    }

    pub fn add_users<'err>(&mut self, users: Vec<User>) -> error::Result<'err, ()>
    {
        for user in &users
        {
            if self.is_user_part_of_server(&user.id)
            {
                return Err(
                    server_error!(error::Kind::AlreadyPartOf, error::OnType::Server)
                        .add_debug_info("user id", user.id.to_string()),
                );
            }
        }

        self.users
            .extend(users.into_iter().map(|user| (user.id.to_string(), user)));

        Ok(())
    }

    pub fn is_server_meeting_requirements<'err>(&self) -> error::Result<'err, ()>
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
        let filtered_channels = if self
            .internal_server_check_permision(user_id, Role::can_read_channels)
        {
            self.channels
                .clone()
                .into_iter()
                .filter(|(channel_id, _)| self.can_read(user_id, Some(channel_id)).unwrap_or(false))
                .collect()
        }
        else
        {
            HashMap::new()
        };

        Self::convert(
            self.id,
            self.name,
            self.owner,
            self.users,
            filtered_channels,
            self.roles,
            self.user_roles,
        )
    }
}

impl channel::Parent for Server
{
    fn get_channel<'input, 'err>(
        &'input self,
        channel_id_option: Option<&'input str>,
    ) -> error::Result<'err, &'input Channel>
    {
        match channel_id_option
        {
            Some(id) => self
                .channels
                .get(id)
                .ok_or(server_error!(error::Kind::NotFound, error::OnType::Channel)),
            None => Err(server_error!(error::Kind::NotFound, error::OnType::Channel)),
        }
    }

    fn get_user_roles(&self, user_id: &str) -> Option<&Vec<String>>
    {
        self.user_roles.get(user_id)
    }

    fn can_read<'input, 'err>(
        &'input self,
        user_id: &'input str,
        channel_id_option: Option<&'input str>,
    ) -> error::Result<'err, bool>
    {
        self.internal_channel_check_permission(user_id, channel_id_option, Channel::can_role_read)
    }

    fn can_write<'input, 'err>(
        &'input self,
        user_id: &'input str,
        channel_id_option: Option<&'input str>,
    ) -> error::Result<'err, bool>
    {
        self.internal_channel_check_permission(user_id, channel_id_option, Channel::can_role_write)
    }
}

impl Server
{
    fn internal_server_check_permision(
        &self,
        user_id: &str,
        func: impl Fn(&Role) -> Option<bool>,
    ) -> bool
    {
        if self.is_owner(user_id)
        {
            return true;
        }

        if !self.users.contains_key(user_id)
        {
            return false;
        }

        let user_roles_option = self.get_user_roles(user_id);

        let roles_default: &Vec<String> = &Vec::new();
        let user_roles: &Vec<String> = user_roles_option.unwrap_or(roles_default);

        for (name, role) in &self.roles
        {
            if name == ROLE_NAME_EVERYBODY && role.can_read_channels().unwrap_or(true)
            {
                return true;
            }

            if !user_roles.contains(name)
            {
                continue;
            }

            if let Some(b) = func(role)
            {
                return b;
            }
        }

        self.roles.is_empty()
    }

    fn internal_channel_check_permission<'err>(
        &self,
        user_id: &str,
        channel_id_option: Option<&str>,
        access_check: impl Fn(&Channel, &str) -> bool,
    ) -> error::Result<'err, bool>
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
            .ok_or(server_error!(error::Kind::NotFound, error::OnType::Channel))?;

        let channel = self
            .channels
            .get(channel_id)
            .ok_or(server_error!(error::Kind::NotFound, error::OnType::Channel))?;

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
