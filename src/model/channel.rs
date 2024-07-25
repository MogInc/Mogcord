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

use super::ROLE_NAME_EVERYBODY;

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
    pub fn new(name: Option<String>, add_base_roles: bool) -> Self
    {
        let name_sanitized = name.map(|name| name.trim().to_owned());

        let mut roles = HashSet::new();

        if add_base_roles
        {
            let role = Role::new(crate::model::ROLE_NAME_EVERYBODY.to_string(), 1);
            roles = HashSet::from([role]);
        }

        Self
        {
            id: Uuid::now_v7().to_string(),
            name: name_sanitized,
            roles,
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

    #[must_use]
    pub fn can_role_read(&self, role_name: &str) -> bool
    {
        for role in &self.roles
        {
            if role.name == ROLE_NAME_EVERYBODY && role.can_read().unwrap_or(true) 
            {
                return true;
            }
    
            if role.name != role_name
            {
                continue;
            }
    
            if let Some(b) = role.can_read()
            {
                return b;
            }
        }

        false
    }

    #[must_use]
    pub fn can_role_write(&self, role_name: &str) -> bool
    {
        for role in &self.roles
        {
            if role.name == ROLE_NAME_EVERYBODY && role.can_read().unwrap_or(true) 
            {
                return true;
            }
    
            if role.name != role_name
            {
                continue;
            }
    
            if let Some(b) = role.can_read()
            {
                return b;
            }
        }

        false
    }
}
