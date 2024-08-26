use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::model::channel::{self, Channel};
use crate::model::error;
use crate::model::user::User;
use crate::server_error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Private
{
    pub id: String,
    pub owners: Vec<User>,
    pub channel: Channel,
}

impl Private
{
    #[must_use]
    fn convert(
        id: String,
        owners: Vec<User>,
        channel: Channel,
    ) -> Self
    {
        Self {
            id,
            owners,
            channel,
        }
    }

    pub fn new<'err>(owners: Vec<User>) -> error::Result<'err, Self>
    {
        let set: HashSet<User> = owners.into_iter().collect();

        let owners_sanitized: Vec<User> = set.into_iter().collect();

        let channel = Channel::new(None, false);

        let private_chat = Private::convert(channel.id.to_string(), owners_sanitized, channel);

        private_chat.internal_is_meeting_requirements()?;

        Ok(private_chat)
    }
}

impl Private
{
    const PRIVATE_OWNER_MAX: usize = 2;

    #[must_use]
    pub fn owner_size() -> usize
    {
        Self::PRIVATE_OWNER_MAX
    }

    #[must_use]
    pub fn is_owner(
        &self,
        user_id: &str,
    ) -> bool
    {
        self.owners.iter().any(|user| user.id == user_id)
    }

    fn internal_is_meeting_requirements<'err>(&self) -> error::Result<'err, ()>
    {
        if !self.internal_is_owner_size_allowed()
        {
            return Err(
                server_error!(error::Kind::InValid, error::OnType::User).add_public_info(format!(
                    "Expected: {}, found: {}",
                    Self::PRIVATE_OWNER_MAX,
                    self.owners.len()
                )),
            );
        }

        Ok(())
    }
    fn internal_is_owner_size_allowed(&self) -> bool
    {
        self.owners.len() == Self::PRIVATE_OWNER_MAX
    }
}

impl channel::Parent for Private
{
    fn get_channel<'input, 'err>(
        &'input self,
        _: Option<&'input str>,
    ) -> error::Result<'err, &'input Channel>
    {
        Ok(&self.channel)
    }

    fn get_user_roles(
        &self,
        _: &str,
    ) -> Option<&Vec<String>>
    {
        None
    }

    fn can_read<'input, 'err>(
        &'input self,
        user_id: &'input str,
        _: Option<&'input str>,
    ) -> error::Result<'err, bool>
    {
        Ok(self.is_owner(user_id))
    }

    fn can_write<'input, 'err>(
        &'input self,
        user_id: &'input str,
        _: Option<&'input str>,
    ) -> error::Result<'err, bool>
    {
        Ok(self.is_owner(user_id))
    }
}
