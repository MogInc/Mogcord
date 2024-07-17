use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::{message::Message, misc::ServerError};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatInfo
{
    pub id: String,
    pub name: Option<String>,
}

impl ChatInfo
{
    pub fn new(
        name: Option<String>, 
    ) -> Result<Self, ServerError>
    {
        let name_sanitized = match name
        {
            Some(name) => Some(name.trim().to_owned()),
            None => None,
        };

        Ok(
            Self
            {
                id: Uuid::now_v7().to_string(),
                name: name_sanitized,
            }
        )
    }
}


//doubt i need this in model
#[derive(Debug, Serialize, Deserialize)]
pub struct Bucket
{
    pub id: String,
    pub chat: ChatInfo,
    pub date: NaiveDate,
    pub messages: Vec<Message>,
}

impl Bucket
{
    pub fn new(chat: &ChatInfo, date: &DateTime<Utc>) -> Self
    {
        Self
        {
            id: Uuid::now_v7().to_string(),
            chat: chat.clone(),
            date: date.date_naive(),   
            messages: Vec::new(),
        }
    }
}

impl Bucket
{
    pub fn add_message(&mut self, message: Message)
    {
        self.messages.push(message);
    }
}