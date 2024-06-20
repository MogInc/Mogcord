use mongodb::bson::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolMessage
{
    pub _id : Uuid,
    pub owner_id: Uuid,
    pub value: String,
    pub chat_id: Uuid,
    pub bucket_id: Uuid,
}

impl MongolMessage
{

}