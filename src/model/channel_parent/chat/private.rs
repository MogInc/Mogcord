use std::collections::HashSet;
use serde::{Deserialize, Serialize};

use crate::model::{channel::{self, Channel}, error, user::User};

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
    fn convert(id: String, owners: Vec<User>, channel: Channel) -> Self
    {
        Self
        {
            id,
            owners,
            channel,
        }
    }

    pub fn new<'stack>(owners: Vec<User>) -> Result<Self, error::Server<'stack>> 
    {
        let set: HashSet<User> = owners
            .into_iter()
            .collect();

        let owners_sanitized: Vec<User> = set
            .into_iter()
            .collect();

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
    pub fn is_owner(&self, user_id: &str) -> bool
    {
        self.owners.iter().any(|user| user.id == user_id)
    }

    fn internal_is_meeting_requirements<'stack>(&self) -> Result<(), error::Server<'stack>> 
    {
        if !self.internal_is_owner_size_allowed()
        {
            return Err(error::Server::new(
                error::Kind::InValid,
                error::OnType::User,
                file!(),
                line!())
                .expose_public_extra_info(format!("Expected: {}, found: {}", Self::PRIVATE_OWNER_MAX, self.owners.len()))
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
    fn get_channel<'input, 'stack>(
        &'input self, 
        _: Option<&'input str>
    ) -> Result<&'input Channel, error::Server<'stack>> 
    {
        Ok(&self.channel)
    }

    fn get_user_roles(&self, _: &str) -> Option<&Vec<String>> 
    {
        None
    }

    fn can_read<'input, 'stack>(
        &'input self, 
        user_id: &'input str, 
        _: Option<&'input str>
    ) -> Result<bool, error::Server<'stack>> 
    {
        Ok(self.is_owner(user_id))
    }

    fn can_write<'input, 'stack>(
        &'input self, 
        user_id: &'input str, 
        _: Option<&'input str>
    ) -> Result<bool, error::Server<'stack>> 
    {
        Ok(self.is_owner(user_id))
    }
}