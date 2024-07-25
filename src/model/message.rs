mod flag;
mod repository;

pub use flag::*;
pub use repository::*;

use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::channel::Channel;
use super::{channel_parent, error};
use super::user::User;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message
{
    pub id: String,
    pub value: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub timestamp: DateTime<Utc>,
    pub owner: User,
    pub channel: Channel,
    pub bucket_id: Option<String>,
    //we actually gonna delete stuff?
    //(:sins:)
    pub flag: Flag,
}

impl Message {
    #[must_use]
    pub fn new(
        value: String, 
        owner: User,
        channel: Channel
    ) -> Self
    {
        Self
        {
            id: Uuid::now_v7().to_string(),
            value,
            timestamp: Utc::now(),
            owner,
            channel,
            bucket_id: None,
            flag: Flag::None
        }
    }
}

impl Message
{
    pub fn update_value(&mut self, value: String, user_id: &str, user_roles: Option<&channel_parent::Roles>) -> Result<(), error::Server>
    {
        if !self.is_user_allowed_to_edit_message(user_id, user_roles)
        {
            return Err(error::Server::MessageNotAllowedToBeEdited);
        }

        if self.value == value
        {
            //can be return Ok(());
            return Err(error::Server::MessageNotAllowedToBeEdited);
        }

        self.value = value;
        self.flag = Flag::Edited { date: Utc::now() };

        Ok(())
    }

    #[must_use]
    pub fn is_channel_part_of_message(&self, channel_id: &str) -> bool
    {
        self.channel.id == *channel_id
    }

    #[must_use]
    pub fn is_user_allowed_to_edit_message(&self, user_id: &str, user_roles_option: Option<&channel_parent::Roles>) -> bool
    {
        if self.owner.id != *user_id || !self.flag.is_allowed_to_be_editted()
        {
            return false;
        }

        let can_read = user_roles_option
            .map_or(!self.channel.has_roles(), |user_roles|
            {
                user_roles
                    .get_all()
                    .iter()
                    .any(|user_role| self.channel.can_role_read(&user_role.name))
            });

        let can_write = user_roles_option
            .map_or(!self.channel.has_roles(), |user_roles|
            {
                user_roles
                    .get_all()
                    .iter()
                    .any(|user_role| self.channel.can_role_write(&user_role.name))
            });

        can_read && can_write
    }
}