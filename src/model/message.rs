mod flag;
mod repository;

pub use flag::*;
pub use repository::*;

use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::channel::Channel;
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
    pub fn update_value(&mut self, value: String)
    {
        if self.value == value
        {
            return;
        }

        self.value = value;
        self.flag = Flag::Edited { date: Utc::now() };
    }

    #[must_use]
    pub fn is_chat_part_of_message(&self, channel_id: &str) -> bool
    {
        self.channel.id == *channel_id
    }

    #[must_use]
    pub fn is_user_allowed_to_edit_message(&self, user_id: &str) -> bool
    {
        //can add more checks since servers can have users with rights etc.
        //(will never be implemented)
        self.owner.id == *user_id && self.flag.is_allowed_to_be_editted()
    }
}