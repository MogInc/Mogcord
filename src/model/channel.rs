mod rights;
mod role;
mod parent;
mod repository;


pub use rights::*;
pub use role::*;
pub use parent::*;
pub use repository::*;

use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Channel
{
    pub id: String,
    pub name: Option<String>,
    pub roles: HashSet<Role>,
}

impl Channel
{
    #[must_use]
    pub fn new(name: Option<String>) -> Self
    {
        let name_sanitized = name.map(|name| name.trim().to_owned());

        Self
        {
            id: Uuid::now_v7().to_string(),
            name: name_sanitized,
            roles: HashSet::new(),
        }
    }

    #[must_use]
    pub fn convert(id: String, name: Option<String>, roles: HashSet<Role>) -> Self
    {
        Self
        {
            id,
            name,
            roles,
        }
    }
}
