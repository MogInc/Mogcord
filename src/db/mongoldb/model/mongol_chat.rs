use mongodb::bson::Uuid;
use serde::{Serialize, Deserialize};
use crate::model::chat::ChatType;

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

}