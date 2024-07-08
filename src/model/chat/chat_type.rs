use serde::{de::{self, Visitor}, Deserialize, Serialize};
use strum_macros::Display;

use crate::model::misc::ServerError;

#[derive(Clone, Display, Debug, Serialize)]
pub enum ChatType
{
    Private,
    Group,
    Server,
}

impl<'de> Deserialize<'de> for ChatType
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> 
    {
        struct ChatTypeVisitor;

        impl<'de> Visitor<'de> for ChatTypeVisitor
        {
            type Value = ChatType;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result 
            {
                return formatter.write_str("data");
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where E: serde::de::Error, 
            {
                match v.to_lowercase().as_str()
                {
                    "private" => Ok(ChatType::Private),
                    "group" => Ok(ChatType::Group),
                    "server" => Ok(ChatType::Server),
                    _ => Err(de::Error::unknown_field(v, FIELDS))
                }
            }
        }

        const FIELDS: &[&str] = &["private", "group", "server"];
        
        deserializer.deserialize_identifier(ChatTypeVisitor)
    }


}

impl ChatType
{
    fn get_requirements(&self) -> Result<ChatTypeRequirements, ServerError>
    {
        #[allow(unreachable_patterns)]
        match self
        {
            ChatType::Private => Ok(
                ChatTypeRequirements
                {
                    owners_count: 2,
                    has_name: false,
                    has_users: false,
                }
            ),
            ChatType::Group => Ok(
                ChatTypeRequirements
                {
                    owners_count: 1,
                    has_name: false,
                    has_users: true,
                }
            ),
            ChatType::Server => Ok(
                ChatTypeRequirements
                {
                    owners_count: 0,
                    has_name: true,
                    has_users: false,
                }
            ),
            _ => Err(ServerError::NotImplemented),
        }
    }

    pub fn is_owner_size_allowed(&self, owner_count: usize) -> bool
    {
        match self.get_requirements() 
        {
            Ok(valid_requirements) => 
                valid_requirements.owners_count == owner_count,
            Err(_) => false,
        }
    }

    pub fn is_chat_meeting_requirements(&self, requirements: ChatTypeRequirements) -> Result<(), ServerError> {
        let valid_requirements = self.get_requirements()?;

        if valid_requirements.owners_count != requirements.owners_count 
        {
            return Err(ServerError::OwnersCountInvalid 
            {
                expected: valid_requirements.owners_count,
                found: requirements.owners_count,
            });
        }

        if valid_requirements.has_name != requirements.has_name 
        {
            return Err(ServerError::NameRequirementInvalid 
            {
                expected: valid_requirements.has_name,
                found: requirements.has_name,
            });
        }

        if valid_requirements.has_users != requirements.has_users 
        {
            return Err(ServerError::UsersRequirementInvalid 
            {
                expected: valid_requirements.has_users,
                found: requirements.has_users,
            });
        }

        Ok(())
    }
}

pub struct ChatTypeRequirements
{
    owners_count: usize,
    has_name: bool,
    has_users: bool,
}

impl ChatTypeRequirements
{
    pub fn new(owners_count: usize, has_name: bool, has_users: bool) -> Self
    {
        Self
        {
            owners_count: owners_count,
            has_name: has_name,
            has_users: has_users,
        }
    }
}

impl PartialEq for ChatTypeRequirements {
    fn eq(&self, other: &Self) -> bool {
        self.owners_count == other.owners_count
        && self.has_name == other.has_name
        && self.has_users == other.has_users
    }
}
