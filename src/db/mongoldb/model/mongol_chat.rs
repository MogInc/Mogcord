use mongodb::bson::{Bson, Uuid};
use serde::{Serialize, Deserialize};
use crate::{db::mongoldb::mongol_helper, model::chat::{Chat, ChatType}};

use super::MongolError;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolChat
{
    pub _id : Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub r#type: ChatType,
    pub owner_ids: Vec<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<Uuid>>,
}

impl TryFrom<&Chat> for MongolChat
{
    type Error = MongolError;

    fn try_from(value: &Chat) -> Result<Self, Self::Error>
    {
        let chat_id = mongol_helper::convert_domain_id_to_mongol(&value.id)?;
        
        let owner_ids = value.owners
            .iter()
            .map(|owner| mongol_helper::convert_domain_id_to_mongol(&owner.id))
            .collect::<Result<_, _>>()?;

        let user_ids = value.users
            .as_ref()
            .map(|users| {
                    users.iter()
                    .map(|user| mongol_helper::convert_domain_id_to_mongol(&user.id))
                    .collect::<Result<_, _>>()
            }).transpose()?;

        Ok(
            Self 
            {
                _id: chat_id,
                name: value.name.clone(),
                r#type: value.r#type.clone(),
                owner_ids: owner_ids,
                user_ids: user_ids,
            }
        )
    }
}

impl From<ChatType> for Bson 
{
    fn from(chat_type: ChatType) -> Bson 
    {
        Bson::String(chat_type.to_string())
    }
}