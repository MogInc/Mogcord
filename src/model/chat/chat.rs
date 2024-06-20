use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::user::User;
use super::MessageFlag;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChatType
{
    Private,
    Group,
    Server,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chat
{
    pub uuid: String,
    pub name: Option<String>,
    pub r#type: ChatType,
    pub owners: Vec<User>,
    pub members: Option<Vec<User>>,
    pub buckets: Option<Vec<Bucket>>,
}

impl Chat
{
    pub fn new(
        name: Option<String>, 
        r#type: ChatType, 
        owners: Vec<User>,
        members: Option<Vec<User>>,
        buckets: Option<Vec<Bucket>>) 
        -> Self
    {
        //TODO: add some buzniz checks
        Self
        {
            uuid: Uuid::new_v4().to_string(),
            name: name,
            r#type: r#type,
            owners: owners,
            members: members,
            buckets: buckets,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bucket
{
    pub uuid: String,
    pub chat: Chat,
    pub date: DateTime<Utc>,
    pub messages: Option<Vec<Message>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message
{
    pub uuid: String,
    pub value: String,
    pub owner: User,
    pub chat: Chat,
    pub bucket: Bucket,
    //we actually gonna delete stuff?
    //(:sins:)
    pub flag: MessageFlag,
}