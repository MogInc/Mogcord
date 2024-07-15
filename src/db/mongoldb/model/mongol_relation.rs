use bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolRelation
{
    user_id: Uuid,
    friend_ids: Vec<Uuid>,
    blocked_ids: Vec<Uuid>,
}

impl MongolRelation
{
    pub fn new(user_id: Uuid) -> Self
    {
        Self
        {
            user_id: user_id,
            friend_ids: Vec::new(),
            blocked_ids: Vec::new(),
        }
    }
}