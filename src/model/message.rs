mod flag;
mod repository;

pub use flag::*;
pub use repository::*;

use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::chat;
use super::user::User;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message
{
    pub id: String,
    pub value: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub timestamp: DateTime<Utc>,
    pub owner: User,
    pub chat: chat::Info,
    pub bucket_id: Option<String>,
    //we actually gonna delete stuff?
    //(:sins:)
    pub flag: MessageFlag,
}

impl Message {
    #[must_use]
    pub fn new(
        value: String, 
        owner: User,
        chat: chat::Info
    ) -> Self
    {
        Self
        {
            id: Uuid::now_v7().to_string(),
            value,
            timestamp: Utc::now(),
            owner,
            chat,
            bucket_id: None,
            flag: MessageFlag::None
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
        self.flag = MessageFlag::Edited { date: Utc::now() };
    }

    #[must_use]
    pub fn is_chat_part_of_message(&self, chat_id: &String) -> bool
    {
        self.chat.id == *chat_id
    }

    #[must_use]
    pub fn is_user_allowed_to_edit_message(&self, user_id: &String) -> bool
    {
        //can add more checks since servers can have users with rights etc.
        //(will never be implemented)
        self.owner.id == *user_id && self.flag.is_allowed_to_be_editted()
    }
}