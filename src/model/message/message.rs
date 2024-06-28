use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::{chat::Chat, user::User};

use super::MessageFlag;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message
{
    pub uuid: String,
    pub value: String,
    pub timestamp: DateTime<Utc>,
    pub owner: User,
    pub chat: Chat,
    pub bucket_uuid: Option<String>,
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
            uuid: Uuid::new_v4().to_string(),
            value: value,
            timestamp: Utc::now(),
            owner: owner,
            chat: chat,
            bucket_uuid: None,
            flag: MessageFlag::None
        }
    }
}