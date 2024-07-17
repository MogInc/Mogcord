use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use uuid::Uuid;

use crate::model::{misc::ServerError, user::User};

use super::ChatInfo;

#[derive(Clone, Display, Debug, Serialize, Deserialize)]
pub enum Chat
{
    Private
    {
        id: String,
        owners: Vec<User>,
        chat_info: ChatInfo,
    },
    Group
    {
        id: String,
        name: String,
        owner: User,
        users: Vec<User>,
        chat_info: ChatInfo,
    },
    Server
    {
        id: String,
        name: String,
        owner: User,
        users: Vec<User>,
        chat_infos: Vec<ChatInfo>,
    },
}


impl Chat
{
    pub fn new_private(owners: Vec<User>, chat_info: ChatInfo) -> Result<Self, ServerError> 
    {
        let set: HashSet<User> = owners
            .into_iter()
            .collect();

        let owners_sanitized: Vec<User> = set
            .into_iter()
            .collect();

        let chat_type = Chat::Private 
        { 
            id: chat_info.id.to_string(),
            owners: owners_sanitized,
            chat_info
        };

        chat_type.is_chat_meeting_requirements()?;

        Ok(chat_type)
    }

    pub fn new_group(name: String, owner: User, users: Vec<User>, chat_info: ChatInfo) -> Result<Self, ServerError> 
    {
        let users_sanitized: Vec<User> = users
            .into_iter()
            .filter(|user| user.id != owner.id)
            .collect();

        let chat_type = Chat::Group 
        { 
            id: chat_info.id.to_string(),
            name, 
            owner, 
            users: users_sanitized,
            chat_info
        };

        chat_type.is_chat_meeting_requirements()?;

        Ok(chat_type)
    }

    pub fn new_server(name: String, owner: User, users: Vec<User>, chat_info: ChatInfo) -> Result<Self, ServerError> 
    {
        let chat_type = Chat::Server 
        { 
            id: Uuid::now_v7().to_string(),
            owner, 
            name, 
            users,
            chat_infos: vec![chat_info],
        };

        chat_type.is_chat_meeting_requirements()?;

        Ok(chat_type)
    }

    pub fn chat_info(self, chat_info_id_option: Option<String>) -> Option<ChatInfo>
    {
        match self
        {
            Chat::Private { chat_info, .. } => Some(chat_info),
            Chat::Group { chat_info, ..  } => Some(chat_info),
            Chat::Server { chat_infos, .. } => 
            {
                let chat_info_id = chat_info_id_option?;

                let position_option = chat_infos
                    .iter()
                    .position(|chat_info| chat_info.id == chat_info_id)?;

                Some(chat_infos[position_option].clone())
            },
        }
    }

    pub fn chat_infos(self) -> Option<Vec<ChatInfo>>
    {
        match self
        {
            Chat::Private { .. } => None,
            Chat::Group { .. } => None,
            Chat::Server { chat_infos, .. } => Some(chat_infos),
        }
    }
}

impl Chat
{
    const PRIVATE_OWNER_MAX: usize = 2;
    const GROUP_OWNER_MAX: usize = 1;
    const SERVER_OWNER_MAX: usize = 1;

    pub fn is_owner_size_allowed(&self, owner_count: usize) -> bool
    {
        match self
        {
            Chat::Private{..} => owner_count == Self::PRIVATE_OWNER_MAX,
            Chat::Group{..} => owner_count == Self::GROUP_OWNER_MAX,
            Chat::Server{..} => owner_count == Self::SERVER_OWNER_MAX,
        }
    }

    pub fn is_chat_meeting_requirements(&self) -> Result<(), ServerError> 
    {
        match self
        {
            Chat::Private{owners,..} => 
            {
                let user_len = owners.len();

                if !self.is_owner_size_allowed(user_len)
                {
                    return Err(ServerError::InternalOwnersCountInvalid { expected: Self::PRIVATE_OWNER_MAX, found: user_len });
                }

                Ok(())
            },
            Chat::Group{..} => Ok(()),
            Chat::Server{..} => Ok(()),
        }
    }

    pub fn is_user_part_of_chat(&self, other_user_id: &String) -> bool
    {
        match self
        {
            Chat::Private{owners,..} => owners.iter().any(|owner| &owner.id == other_user_id),
            Chat::Group{owner, users, ..} => &owner.id == other_user_id
                || users.iter().any(|user| &user.id == other_user_id),
            _ => true,
        }
    }
}