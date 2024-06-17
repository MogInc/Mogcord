use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use super::MongolUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolChat
{
    pub _id : ObjectId,
    pub owner: MongolUser,
    pub users: Vec<MongolUser>,
    pub bucket_ids: Option<Vec<String>>
}

impl MongolChat
{

}