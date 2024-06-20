use mongodb::bson::Uuid;
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

impl TryFrom<Chat> for MongolChat
{
    type Error = MongolError;

    fn try_from(value: Chat) -> Result<Self, Self::Error>
    {
        let chat_id =  Uuid::parse_str(&value.uuid)
                             .map_err(|_| MongolError::InvalidUUID)?;
        
        let owner_ids = value.owners
                                    .into_iter()
                                    .map(|owner| Uuid::parse_str(&owner.uuid).map_err(|_| MongolError::InvalidUUID))
                                    .collect::<Result<_, _>>()?;

        let user_ids = value.members
        .map(|members| {
            members.into_iter()
                .map(|member| Uuid::parse_str(&member.uuid).map_err(|_| MongolError::InvalidUUID))
                .collect::<Result<_, _>>()
        }).transpose()?;

        let bucket_ids = value.buckets
        .map(|buckets| {
            buckets.into_iter().map(|bucket| Uuid::parse_str(&bucket.uuid).map_err(|_| MongolError::InvalidUUID))
            .collect::<Result<_,_>>()
        }).transpose()?;

        Ok(
            Self 
            {
                _id: chat_id,
                name: value.name,
                r#type: value.r#type,
                owner_ids: owner_ids,
                user_ids: user_ids,
                bucket_ids: bucket_ids,
            }
        )
    }
}
