use serde::{
    Deserialize,
    Serialize,
};
use std::cmp::Ordering;
use strum::IntoEnumIterator;

use super::Rights;

#[derive(Clone, Debug, Serialize, Deserialize)]
//channel roles, roles linked to channels
pub struct Role
{
    pub name: String,
    pub rank: usize,
    rights: Vec<Rights>,
}

impl Role
{
    #[must_use]
    pub fn new_neutral(
        name: String,
        rank: usize,
    ) -> Self
    {
        Self {
            name,
            rank,
            rights: Self::internal_default_rights(),
        }
    }

    #[must_use]
    pub fn new_public(
        name: String,
        rank: usize,
    ) -> Self
    {
        Self {
            name,
            rank,
            rights: Self::internal_default_public_rights(),
        }
    }

    #[must_use]
    pub fn new_private(
        name: String,
        rank: usize,
    ) -> Self
    {
        Self {
            name,
            rank,
            rights: Self::internal_default_private_rights(),
        }
    }

    #[must_use]
    fn internal_default_rights() -> Vec<Rights>
    {
        Rights::iter().collect()
    }

    #[must_use]
    fn internal_default_public_rights() -> Vec<Rights>
    {
        Rights::iter()
            .map(|right| match right
            {
                Rights::Read(_) => Rights::Read(Some(true)),
                Rights::Write(_) => Rights::Write(Some(true)),
            })
            .collect()
    }

    #[must_use]
    fn internal_default_private_rights() -> Vec<Rights>
    {
        Rights::iter()
            .map(|right| match right
            {
                Rights::Read(_) => Rights::Read(Some(false)),
                Rights::Write(_) => Rights::Write(Some(false)),
            })
            .collect()
    }
}

impl Role
{
    #[must_use]
    pub fn can_read(&self) -> Option<bool>
    {
        self.rights.iter().find_map(|right| {
            if let Rights::Read(value) = right
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
    pub fn can_write(&self) -> Option<bool>
    {
        self.rights.iter().find_map(|right| {
            if let Rights::Write(value) = right
            {
                Some(*value)
            }
            else
            {
                None
            }
        })?
    }

    pub fn add_right(
        &mut self,
        right: Rights,
    )
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

    pub fn remove_right(
        &mut self,
        right: &Rights,
    )
    {
        self.rights.retain(|r| r != right);
    }
}

impl std::hash::Hash for Role
{
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    )
    {
        self.name.hash(state);
    }
}

impl Ord for Role
{
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering
    {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Role
{
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl PartialEq for Role
{
    fn eq(
        &self,
        other: &Self,
    ) -> bool
    {
        self.name == other.name
    }
}
impl Eq for Role {}
