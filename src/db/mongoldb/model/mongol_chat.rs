use mongodb::bson::{Bson, Uuid};
use serde::{Serialize, Deserialize};
use crate::{db::mongoldb::mongol_helper, model::{chat::{Chat, ChatType}, misc::ServerError}};

use super::MongolChatType;


#[derive(Debug, Serialize, Deserialize)]
pub struct MongolChat
{
    pub _id : Uuid,
    pub name: Option<String>,
    pub r#type: MongolChatType,
}

impl TryFrom<&Chat> for MongolChat
{
    type Error = ServerError;

    fn try_from(value: &Chat) -> Result<Self, Self::Error>
    {
        let chat_id = mongol_helper::convert_domain_id_to_mongol(&value.id)?;

        Ok(
            Self 
            {
                _id: chat_id,
                name: value.name.clone(),
                r#type: MongolChatType::try_from(&value.r#type)?,
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