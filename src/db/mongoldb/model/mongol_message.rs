use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use super::MongolUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolMessage
{
    pub _id : ObjectId,
    pub owner: MongolUser,
    pub value: String,
    pub chat_id: ObjectId,
    pub bucket_id: ObjectId,
}

impl MongolMessage
{

}