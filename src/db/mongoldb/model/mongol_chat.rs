use mongodb::bson::{Bson, Uuid};
use serde::{Serialize, Deserialize};
use crate::model::chat::{Chat, ChatType};

use super::MongolError;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolChat
{
    pub _id : Uuid,
    pub name: Option<String>,
    pub r#type: ChatType,
    pub owner_ids: Vec<Uuid>,
    pub user_ids: Option<Vec<Uuid>>,
    pub bucket_ids: Option<Vec<Uuid>>
}

impl MongolChat
{
    pub fn new(value: Chat) -> Result<Self, MongolError>
    {
        let chat_id: Uuid =  Uuid::parse_str(&value.uuid)
            .map_err(|_| MongolError::InvalidUUID)?;
        
        let owner_ids: Vec<Uuid> = value.owners
            .into_iter()
            .map(|owner| Uuid::parse_str(&owner.uuid).map_err(|_| MongolError::InvalidUUID))
            .collect::<Result<_, _>>()?;

        let user_ids: Option<Vec<Uuid>> = value.users
            .map(|users| {
                    users.into_iter()
                    .map(|user| Uuid::parse_str(&user.uuid).map_err(|_| MongolError::InvalidUUID))
                    .collect::<Result<_, _>>()
            }).transpose()?;

        Ok(
            Self 
            {
                _id: chat_id,
                name: value.name,
                r#type: value.r#type,
                owner_ids: owner_ids,
                user_ids: user_ids,
                bucket_ids: None,
            }
        )
    }
}

impl From<ChatType> for Bson {
    fn from(chat_type: ChatType) -> Bson {
        // Add your conversion logic here
        Bson::String(chat_type.to_string())
    }
}