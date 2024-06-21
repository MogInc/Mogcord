use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::user::User;
use super::{ChatError, MessageFlag};

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
        members: Option<Vec<User>>) 
        -> Result<Self, ChatError>
    {

        if !Self::is_owner_size_allowed(&r#type, owners.len())
        {
            return Err(ChatError::InvalidOwnerCount);
        }

        let members: Option<Vec<User>> = members.map(|members| {
            members.into_iter().filter(|x| !owners.contains(x)).collect()
        });

        Ok(Self{
            uuid: Uuid::new_v4().to_string(),
            name: name,
            r#type: r#type,
            owners: owners,
            members: members,
            buckets: None,
        })
    }

    pub fn is_owner_size_allowed(r#type: &ChatType, owner_count: usize) -> bool
    {
        let max_owner_count: usize = match r#type
        {
            ChatType::Private => 2,
            ChatType::Server | ChatType::Group => 1,
        };

        return max_owner_count == owner_count;
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