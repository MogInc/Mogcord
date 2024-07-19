use mongodb::bson::{Bson, Uuid};
use serde::{Serialize, Deserialize};
use crate::{db::mongoldb::helper, model::{chat::{ChatInfo, Chat}, error}};


#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::pub_underscore_fields)]
#[allow(clippy::used_underscore_binding)]
pub struct MongolChatInfo
{
    pub _id : Uuid,
    pub name: Option<String>,
}

impl TryFrom<&ChatInfo> for MongolChatInfo
{
    type Error = error::Server;

    fn try_from(value: &ChatInfo) -> Result<Self, Self::Error>
    {
        let chat_id = helper::convert_domain_id_to_mongol(&value.id)?;

        Ok(
            Self 
            {
                _id: chat_id,
                name: value.name.clone(),
            }
        )
    }
}

pub struct MongolChatInfoWrapper(pub Vec<MongolChatInfo>);

impl TryFrom<&Vec<ChatInfo>> for MongolChatInfoWrapper
{
    type Error = error::Server;

    fn try_from(value: &Vec<ChatInfo>) -> Result<Self, Self::Error>
    {
        let mut chat_info_vec = Vec::new();

        for chat_info in value
        {
            chat_info_vec.push(MongolChatInfo::try_from(chat_info)?);
        }

        Ok(MongolChatInfoWrapper(chat_info_vec))
    }
}

impl From<Chat> for Bson 
{
    fn from(chat: Chat) -> Bson 
    {
        Bson::String(chat.to_string())
    }
}