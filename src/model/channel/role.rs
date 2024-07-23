use serde::{Deserialize, Serialize};
use strum::{EnumCount, IntoEnumIterator};

use super::Rights;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role
{
    pub name: String,
    pub rank: usize,
    rights: Vec<Rights>,
}

impl Role
{
    #[must_use]
    pub fn new(name: String, rank: usize) -> Self
    {
        Self
        {
            name,
            rank,
            rights: Self::default_rights(),
        }
    }

    #[must_use]
    pub fn new_private(name: String, rank: usize) -> Self
    {
        Self
        {
            name,
            rank,
            rights: Self::default_private_rights(),
        }
    }
}

impl Role
{
    #[must_use]
    pub fn can_read(&self) -> Option<bool>
    {
        self.rights
            .iter()
            .find_map(|right| 
                if let Rights::Read(value) = right
                {
                    Some(*value)
                }
                else
                {
                    None
                }
            )?
    }

    #[must_use]
    pub fn can_write(&self) -> Option<bool>
    {
        self.rights
            .iter()
            .find_map(|right| 
                if let Rights::Write(value) = right
                {
                    Some(*value)
                }
                else
                {
                    None
                }
            )?
    }

    #[must_use]
    pub fn default_rights() -> Vec<Rights>
    {
        Rights::iter().collect()
    }

    #[must_use]
    pub fn default_private_rights() -> Vec<Rights>
    {
        Rights::iter().map(|right| {
            match right
            {
                Rights::Read(_) => Rights::Read(Some(false)),
                Rights::Write(_) => Rights::Write(Some(false)),
            }
        }).collect()
    }

    pub fn add_right(&mut self, right: Rights) 
    {
        if let Some(pos) = self.rights.iter().position(|r| r == &right) 
        {
            self.rights[pos] = right;
        }
        else 
        {
            self.rights.push(right);
        }
    }

    pub fn remove_right(&mut self, right: &Rights) 
    {
        self
            .rights
            .retain(|r| r != right);
    }
}

impl std::hash::Hash for Role
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) 
    {
        self.name.hash(state);
    }
}

impl PartialEq for Role
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.name == other.name
    }
}
impl Eq for Role {}