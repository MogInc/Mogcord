use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::{chat::Chat, user::User};

use super::MessageFlag;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message
{
    pub id: String,
    pub value: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub timestamp: DateTime<Utc>,
    pub owner: User,
    pub chat: Chat,
    pub bucket_id: Option<String>,
    //we actually gonna delete stuff?
    //(:sins:)
    pub flag: MessageFlag,
}

impl Message {
    pub fn new(
        value: String, 
        owner: User,
        chat: Chat
    ) -> Self
    {
        Self
        {
            id: Uuid::now_v7().to_string(),
            value: value,
            timestamp: Utc::now(),
            owner: owner,
            chat: chat,
            bucket_id: None,
            flag: MessageFlag::None
        }
    }
}

impl Message
{
    pub fn update_value(&mut self, value: String)
    {
        self.value = value;
    }

    pub fn is_chat_part_of_message(&self, chat_id: &String) -> bool
    {
        return self.chat.id == *chat_id;
    }
    pub fn is_user_allowed_to_edit_message(&self, user_id: &String) -> bool
    {
        //can add more checks since servers can have users with rights etc.
        //(will never be implemented)
        return self.owner.id == *user_id;
    }
}