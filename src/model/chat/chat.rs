use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::{error::ServerError, user::User};
use super::{chat_type::{ChatType, ChatTypeRequirements}, MessageFlag};


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
        -> Result<Self, ServerError>
    {

        let members_sanitized: Option<Vec<User>> = members.map(|members| {
            members.into_iter().filter(|x| !owners.contains(x)).collect()
        });
        
        let requirements = ChatTypeRequirements::new(
            owners.len(), 
            name.as_ref().is_some_and(|x| !x.trim().is_empty()), 
            members_sanitized.as_ref().is_some_and(|x| x.len() > 0)
        );

        if let Err(err) = r#type.is_chat_meeting_requirements(requirements)
        {
            return Err(err);
        }

        let name_sanitized = match name
        {
            Some(name) => Some(name.trim().to_owned()),
            None => None,
        };

        let members_sanitized = match members_sanitized
        {
            Some(members) if members.is_empty() => None,
            _ => members_sanitized
        };

        Ok(Self{
            uuid: Uuid::new_v4().to_string(),
            name: name_sanitized,
            r#type: r#type,
            owners: owners,
            members: members_sanitized,
            buckets: None,
        })
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