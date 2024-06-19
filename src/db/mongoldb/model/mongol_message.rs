use mongodb::bson::Uuid;
use serde::{Serialize, Deserialize};
use super::MongolUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolMessage
{
    pub _id : Uuid,
    pub owner: MongolUser,
    pub value: String,
    pub chat_id: Uuid,
    pub bucket_id: Uuid,
}

impl MongolMessage
{

}