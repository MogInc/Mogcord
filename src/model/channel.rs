mod parent;
mod repository;
mod rights;
mod role;

pub use parent::*;
pub use repository::*;
pub use rights::*;
pub use role::*;

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use uuid::Uuid;

use super::ROLE_NAME_EVERYBODY;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Channel
{
    pub id: String,
    pub name: Option<String>,
    pub roles: BTreeSet<Role>,
}

impl Channel
{
    #[must_use]
    pub fn new(
        name: Option<String>,
        add_base_roles: bool,
    ) -> Self
    {
        let name_sanitized = name.map(|name| name.trim().to_owned());

        let mut roles = BTreeSet::new();

        if add_base_roles
        {
            let role = Role::new_neutral(
                crate::model::ROLE_NAME_EVERYBODY.to_string(),
                1,
            );
            roles = BTreeSet::from([role]);
        }

        Self {
            id: Uuid::now_v7().to_string(),
            name: name_sanitized,
            roles,
        }
    }

    #[must_use]
    pub fn new_private(name: Option<String>) -> Self
    {
        let name_sanitized = name.map(|name| name.trim().to_owned());

        let role = Role::new_private(
            crate::model::ROLE_NAME_EVERYBODY.to_string(),
            1,
        );
        let roles = BTreeSet::from([role]);

        Self {
            id: Uuid::now_v7().to_string(),
            name: name_sanitized,
            roles,
        }
    }

    #[must_use]
    pub fn convert(
        id: String,
        name: Option<String>,
        roles: BTreeSet<Role>,
    ) -> Self
    {
        Self {
            id,
            name,
            roles,
        }
    }

    pub fn add_role(
        &mut self,
        mut role: Role,
    )
    {
        let rank = (self.roles.len() + 1).min(role.rank);
        role.rank = rank;

        let mut updated_roles = BTreeSet::new();

        for existing_role in &self.roles
        {
            if existing_role.rank >= role.rank
            {
                let mut adjusted_role = existing_role.clone();
                adjusted_role.rank += 1;
                updated_roles.insert(adjusted_role);
            }
            else
            {
                updated_roles.insert(existing_role.clone());
            }
        }

        updated_roles.insert(role);

        self.roles = updated_roles;
    }

    #[must_use]
    /// returns `true` or `false` if the role has read rights.
    ///
    /// # Examples - pseudo code for simplicity
    /// ```
    /// # use mogcord::model::channel::Channel;
    /// # use mogcord::model::channel::Role;
    /// //channel has roles - everyone = true
    /// let mut channel = Channel::new(Some(String::from("Channel")), true);
    /// channel.add_role(Role::new_private(String::from("a"), 2));
    /// //{
    /// //    { role: name: "a", weight: 2, read: false }
    /// //    { role: name: "everyone", weight: 1, read: true }
    /// //}
    /// //can still read regardless if they have role "a"
    /// assert!(channel.can_role_read("a"));
    /// assert!(channel.can_role_read("b"));
    ///
    /// //channel has roles - everyone = false
    /// let mut channel = Channel::new_private(Some(String::from("Channel")));
    /// channel.add_role(Role::new_public(String::from("a"), 2));
    /// //{
    /// //    { role: name: "a", weight: 2, read: true }
    /// //    { role: name: "everyone", weight: 1, read: false }
    /// //}
    ///
    /// // only role "a" can read
    /// assert!(channel.can_role_read("a"));
    /// assert!(!channel.can_role_read("b"));
    ///
    /// let mut channel = Channel::new(Some(String::from("Channel")), false);
    /// //channel has no roles
    /// //{
    /// //}
    /// //can still read regardless of the role
    /// assert!(channel.can_role_read("a"));
    /// assert!(channel.can_role_read("b"));
    /// ```
    pub fn can_role_read(
        &self,
        role_name: &str,
    ) -> bool
    {
        self.internal_can_role_perform_action(role_name, Role::can_read)
    }

    #[must_use]
    /// returns `true` or `false` if the role has write rights.
    ///
    /// # Examples - pseudo code for simplicity
    /// ```
    /// # use mogcord::model::channel::Channel;
    /// # use mogcord::model::channel::Role;
    /// //channel has roles - everyone = true
    /// let mut channel = Channel::new(Some(String::from("Channel")), true);
    /// channel.add_role(Role::new_private(String::from("a"), 2));
    /// //{
    /// //    { role: name: "a", weight: 2, write: false }
    /// //    { role: name: "everyone", weight: 1, write: true }
    /// //}
    /// //can still write regardless if they have role "a"
    /// assert!(channel.can_role_write("a"));
    /// assert!(channel.can_role_write("b"));
    ///
    /// //channel has roles - everyone = false
    /// let mut channel = Channel::new_private(Some(String::from("Channel")));
    /// channel.add_role(Role::new_public(String::from("a"), 2));
    /// //{
    /// //    { role: name: "a", weight: 2, write: true }
    /// //    { role: name: "everyone", weight: 1, write: false }
    /// //}
    ///
    /// // only role "a" can write
    /// assert!(channel.can_role_write("a"));
    /// assert!(!channel.can_role_write("b"));
    ///
    /// let mut channel = Channel::new(Some(String::from("Channel")), false);
    /// //channel has no roles
    /// //{
    /// //}
    /// //can still write regardless of the role
    /// assert!(channel.can_role_write("a"));
    /// assert!(channel.can_role_write("b"));
    /// ```
    pub fn can_role_write(
        &self,
        role_name: &str,
    ) -> bool
    {
        self.internal_can_role_perform_action(role_name, Role::can_write)
    }

    #[must_use]
    pub fn has_roles(&self) -> bool
    {
        !self.roles.is_empty()
    }

    fn internal_can_role_perform_action<T>(
        &self,
        role_name: &str,
        func: T,
    ) -> bool
    where
        T: Fn(&Role) -> Option<bool>,
    {
        for role in &self.roles
        {
            if role.name == ROLE_NAME_EVERYBODY
                && role.can_read().unwrap_or(true)
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
