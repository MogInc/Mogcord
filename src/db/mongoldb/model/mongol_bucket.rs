use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolBucket 
{
    pub _id: ObjectId,
    pub chat_id: ObjectId, 
    pub date: DateTime,
    pub message_ids: Option<Vec<ObjectId>>, 
}

impl MongolBucket
{

}