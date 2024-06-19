use mongodb::bson::Uuid;
use serde::{Serialize, Deserialize};
use crate::model::chat::ChatType;

use super::MongolUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolChat
{
    pub _id : Uuid,
    pub name: Option<String>,
    pub r#type: ChatType,
    pub owner: Vec<MongolUser>,
    pub users: Vec<MongolUser>,
    pub bucket_ids: Option<Vec<String>>
}

impl MongolChat
{

}