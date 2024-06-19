use mongodb::bson::{DateTime, Uuid};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolBucket 
{
    pub _id: Uuid,
    pub chat_id: Uuid, 
    pub date: DateTime,
    pub message_ids: Option<Vec<Uuid>>, 
}

impl MongolBucket
{

}