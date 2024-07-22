use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use super::Rights;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Roles
{
    //key is role name
    roles: HashMap<String, Role>,
}

impl Default for Roles
{
    #[must_use]
    fn default() -> Self
    {
        Self 
        { 
            roles: HashMap::new()
        }
    }
}

impl Roles
{
    #[must_use]
    pub fn contains(&self, key: &str) -> bool
    {
        self.roles.contains_key(key)
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Role>
    {
        self.roles.get(key)
    }

    #[must_use]
    pub fn get_all(&self) -> Vec<Role>
    {
        self.roles.clone().into_values().collect()
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role
{
    pub name: String,
    pub rank: usize,
    rights: HashSet<Rights>,
}

impl Role
{
    #[must_use]
    pub fn get_read_channel(&self) -> Option<&Rights>
    {
        self.rights.get(&Rights::ReadChannels(None))
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
