use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::channel::Channel;
use super::message::Message;

//doubt i need this in model
#[derive(Debug, Serialize, Deserialize)]
pub struct Bucket
{
    pub id: String,
    pub channel: Channel,
    pub date: NaiveDate,
    pub messages: Vec<Message>,
}

impl Bucket
{
    #[must_use]
    pub fn new(channel: &Channel, date: &DateTime<Utc>) -> Self
    {
        Self {
            id: Uuid::now_v7().to_string(),
            channel: channel.clone(),
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
