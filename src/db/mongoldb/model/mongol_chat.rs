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
}

impl TryFrom<Chat> for MongolChat
{
    type Error = MongolError;

    fn try_from(value: Chat) -> Result<Self, Self::Error>
    {
        let chat_id: Uuid =  Uuid::parse_str(&value.id)
            .map_err(|_| MongolError::InvalidID)?;
        
        let owner_ids: Vec<Uuid> = value.owners
            .into_iter()
            .map(|owner| Uuid::parse_str(&owner.id).map_err(|_| MongolError::InvalidID))
            .collect::<Result<_, _>>()?;

        let user_ids: Option<Vec<Uuid>> = value.users
            .map(|users| {
                    users.into_iter()
                    .map(|user| Uuid::parse_str(&user.id).map_err(|_| MongolError::InvalidID))
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