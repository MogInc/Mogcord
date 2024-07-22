use serde::{Deserialize, Serialize};

use super::Rights;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role
{
    name: String,
    rank: usize,
    rights: Vec<Rights>,
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
