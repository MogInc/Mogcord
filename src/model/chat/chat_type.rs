use serde::{Deserialize, Serialize};

use crate::model::error::ServerError;

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
        #[allow(unreachable_patterns)]
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

impl ChatTypeRequirements
{
    pub fn new(owners_count: usize, has_name: bool, has_members: bool) -> Self
    {
        Self
        {
            owners_count: owners_count,
            has_name: has_name,
            has_members: has_members,
        }
    }
}

impl PartialEq for ChatTypeRequirements {
    fn eq(&self, other: &Self) -> bool {
        self.owners_count == other.owners_count
        && self.has_name == other.has_name
        && self.has_members == other.has_members
    }
}
