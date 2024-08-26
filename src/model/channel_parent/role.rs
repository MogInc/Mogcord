use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use super::Rights;

#[derive(Clone, Debug, Serialize, Deserialize)]
//server roles, roles linked to server
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
        Self {
            name,
            rank,
            rights: Self::default_rights(),
        }
    }

    #[must_use]
    pub fn new_private(name: String, rank: usize) -> Self
    {
        Self {
            name,
            rank,
            rights: Self::default_private_rights(),
        }
    }
}

impl Role
{
    #[must_use]
    #[allow(irrefutable_let_patterns)]
    pub fn can_read_channels(&self) -> Option<bool>
    {
        self.rights.iter().find_map(|right| {
            if let Rights::ReadChannels(value) = right
            {
                Some(*value)
            }
            else
            {
                None
            }
        })?
    }

    #[must_use]
    pub fn default_rights() -> Vec<Rights>
    {
        Rights::iter().collect()
    }

    #[must_use]
    pub fn default_private_rights() -> Vec<Rights>
    {
        Rights::iter()
            .map(|right| match right
            {
                Rights::ReadChannels(_) => Rights::ReadChannels(Some(false)),
            })
            .collect()
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
