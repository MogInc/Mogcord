use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::{error::ServerError, user::User};
use super::MessageFlag;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChatType
{
    Private,
    Group,
    Server,
}

impl ChatType
{
    fn get_requirements(&self) -> Result<ChatTypeRequirements, ServerError>
    {
        match self
        {
            ChatType::Private => Ok(
                ChatTypeRequirements
                {
                    owners_count: 2,
                    has_name: false,
                    has_members: false,
                }
            ),
            ChatType::Group => Ok(
                ChatTypeRequirements
                {
                    owners_count: 1,
                    has_name: false,
                    has_members: true,
                }
            ),
            ChatType::Server => Ok(
                ChatTypeRequirements
                {
                    owners_count: 0,
                    has_name: true,
                    has_members: false,
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
            return Err(ServerError::InvalidOwnersCount 
            {
                expected: valid_requirements.owners_count,
                found: requirements.owners_count,
            });
        }

        if valid_requirements.has_name != requirements.has_name 
        {
            return Err(ServerError::InvalidNameRequirement 
            {
                expected: valid_requirements.has_name,
                found: requirements.has_name,
            });
        }

        if valid_requirements.has_members != requirements.has_members 
        {
            return Err(ServerError::InvalidMembersRequirement 
            {
                expected: valid_requirements.has_members,
                found: requirements.has_members,
            });
        }

        Ok(())
    }
}

pub struct ChatTypeRequirements
{
    owners_count: usize,
    has_name: bool,
    has_members: bool,
}

impl PartialEq for ChatTypeRequirements {
    fn eq(&self, other: &Self) -> bool {
        self.owners_count == other.owners_count
        && self.has_name == other.has_name
        && self.has_members == other.has_members
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chat
{
    pub uuid: String,
    pub name: Option<String>,
    pub r#type: ChatType,
    pub owners: Vec<User>,
    pub members: Option<Vec<User>>,
    pub buckets: Option<Vec<Bucket>>,
}

impl Chat
{
    pub fn new(
        name: Option<String>, 
        r#type: ChatType, 
        owners: Vec<User>,
        members: Option<Vec<User>>) 
        -> Result<Self, ServerError>
    {

        let members_sanitized: Option<Vec<User>> = members.map(|members| {
            members.into_iter().filter(|x| !owners.contains(x)).collect()
        });
        
        let requirements = ChatTypeRequirements
        {
            owners_count: owners.len(),
            has_name: name.as_ref().is_some_and(|x| !x.trim().is_empty()),
            has_members: members_sanitized.as_ref().is_some_and(|x| x.len() > 0),
        };

        if let Err(err) = r#type.is_chat_meeting_requirements(requirements)
        {
            return Err(err);
        }

        let name_sanitized = match name
        {
            Some(name) => Some(name.trim().to_owned()),
            None => None,
        };

        let members_sanitized = match members_sanitized
        {
            Some(members) if members.is_empty() => None,
            _ => members_sanitized
        };

        Ok(Self{
            uuid: Uuid::new_v4().to_string(),
            name: name_sanitized,
            r#type: r#type,
            owners: owners,
            members: members_sanitized,
            buckets: None,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bucket
{
    pub uuid: String,
    pub chat: Chat,
    pub date: DateTime<Utc>,
    pub messages: Option<Vec<Message>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message
{
    pub uuid: String,
    pub value: String,
    pub owner: User,
    pub chat: Chat,
    pub bucket: Bucket,
    //we actually gonna delete stuff?
    //(:sins:)
    pub flag: MessageFlag,
}