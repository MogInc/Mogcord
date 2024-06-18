use chrono::{DateTime, Utc};

use crate::model::user::User;

pub enum ChatType
{
    Private,
    Group,
    Server,
}

pub struct Chat
{
    pub uuid: String,
    pub name: Option<String>,
    pub r#type: ChatType,
    pub owners: Vec<User>,
    pub members: Option<Vec<User>>,
    pub buckets: Option<Vec<Bucket>>,
}

pub struct Bucket
{
    pub chat: Chat,
    pub date: DateTime<Utc>,
    pub messages: Option<Vec<Message>>,
}

pub enum MessageFlag
{
    None,
    Edited,
    Deleted,
}

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