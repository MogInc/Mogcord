use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::{chat::{Bucket, Chat}, user::User};

use super::MessageFlag;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message
{
    pub uuid: String,
    pub value: String,
    pub timestamp: DateTime<Utc>,
    pub owner: User,
    pub chat: Chat,
    pub bucket: Option<Bucket>,
    //we actually gonna delete stuff?
    //(:sins:)
    pub flag: MessageFlag,
}