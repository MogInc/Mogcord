use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::{message::Message, misc::ServerError, user::User};
use super::chat_type::{ChatType, ChatTypeRequirements};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chat
{
    pub id: String,
    pub name: Option<String>,
    pub r#type: ChatType,
    pub owners: Vec<User>,
    pub users: Option<Vec<User>>,
}

impl Chat
{
    pub fn new(
        name: Option<String>, 
        r#type: ChatType,
        owners: Vec<User>,
        users: Option<Vec<User>>
    ) -> Result<Self, ServerError>
    {

        let users_sanitized: Option<Vec<User>> = users.map(|users| {
            users.into_iter().filter(|x| !owners.contains(x)).collect()
        });
        
        let requirements = ChatTypeRequirements::new(
            owners.len(), 
            name.as_ref().is_some_and(|x| !x.trim().is_empty()), 
            users_sanitized.as_ref().is_some_and(|x| x.len() > 0)
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

        let users_sanitized = match users_sanitized
        {
            Some(users) if users.is_empty() => None,
            _ => users_sanitized
        };

        Ok(Self{
            id: Uuid::now_v7().to_string(),
            name: name_sanitized,
            r#type: r#type,
            owners: owners,
            users: users_sanitized,
        })
    }
}


impl Chat
{
    pub fn is_user_part_of_chat(&self, user_id: &String) -> bool
    {
        match self.r#type
        {
            ChatType::Private => self.owners.iter().any(|owner| &owner.id == user_id),
            ChatType::Group => self.owners.iter().any(|owner| &owner.id == user_id) 
                || self.users.as_ref().map_or(false, |users|{
                    users.iter().any(|user| &user.id == user_id)
                }),
            _ => true,
        }
    }
}


//doubt i need this in model
#[derive(Debug, Serialize, Deserialize)]
pub struct Bucket
{
    pub id: String,
    pub chat: Chat,
    pub date: NaiveDate,
    pub messages: Option<Vec<Message>>,
}

impl Bucket
{
    pub fn new(chat: &Chat, date: &DateTime<Utc>) -> Self
    {
        Self
        {
            id: Uuid::now_v7().to_string(),
            chat: chat.clone(),
            date: date.date_naive(),   
            messages: None,
        }
    }
}

impl Bucket
{
    pub fn add_message(&mut self, message: Message)
    {
        if self.messages.is_none() 
        {
            self.messages = Some(Vec::new());
        }

        if let Some(messages) = &mut self.messages 
        {
            messages.push(message);
        }
    }
}