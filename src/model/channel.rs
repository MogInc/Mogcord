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
    /// returns `true` or `false` if the role has read rights.
    /// 
    /// # Examples - pseudo code for simplicity
    /// ```
    /// //channel has roles - everyone = true
    /// channel.roles
    /// {
    ///     { role: name: "a", weight: 2, read: false }
    ///     { role: name: "everyone", weight: 1, read: true }
    /// }
    /// //can still read regardless if they have role "a"
    /// assert!(channel.can_role_read("a"))
    /// assert!(channel.can_role_read("b"))
    /// 
    /// //channel has roles - everyone = true
    /// channel.roles
    /// {
    ///     { role: name: "a", weight: 2, read: true }
    ///     { role: name: "everyone", weight: 1, read: false }
    /// }
    /// // only role "a" can read
    /// assert!(channel.can_role_read("a"))
    /// assert!(!channel.can_role_read("b"))
    /// 
    /// //channel has no roles
    /// channel.roles
    /// {
    /// }
    /// //can still read regardless of the role
    /// assert!(channel.can_role_read("a"))
    /// assert!(channel.can_role_read("b"))
    /// ```
    pub fn can_role_read(&self, role_name: &str) -> bool
    {
        self.internal_can_role_perform_action(role_name, Role::can_read)
    }

    #[must_use]
    /// returns `true` or `false` if the role has write rights.
    /// 
    /// # Examples - pseudo code for simplicity
    /// ```
    /// //channel has roles - everyone = true
    /// channel.roles
    /// {
    ///     { role: name: "a", weight: 2, write: false }
    ///     { role: name: "everyone", weight: 1, write: true }
    /// }
    /// //can still write regardless if they have role "a"
    /// assert!(channel.can_role_write("a"))
    /// assert!(channel.can_role_write("b"))
    /// 
    /// //channel has roles - everyone = true
    /// channel.roles
    /// {
    ///     { role: name: "a", weight: 2, write: true }
    ///     { role: name: "everyone", weight: 1, write: false }
    /// }
    /// //only role "a" can write
    /// assert!(channel.can_role_write("a"))
    /// assert!(!channel.can_role_write("b"))
    /// 
    /// //channel has no roles
    /// channel.roles
    /// {
    /// }
    /// //can still write regardless of the role
    /// assert!(channel.can_role_write("a"))
    /// assert!(channel.can_role_write("b"))
    /// ```
    pub fn can_role_write(&self, role_name: &str) -> bool
    {
        self.internal_can_role_perform_action(role_name, Role::can_write)
    }

    fn internal_can_role_perform_action<T>(&self, role_name: &str, func: T) -> bool
    where 
        T: Fn(&Role) -> Option<bool>,
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
    
            if let Some(b) = func(role)
            {
                return b;
            }
        }

        self.roles.is_empty()
    }
}
