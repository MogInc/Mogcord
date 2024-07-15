use bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolRelation
{
    user_id: Uuid,
    friend_ids: Option<Vec<Uuid>>,
    blocked_ids: Option<Vec<Uuid>>,
}

impl MongolRelation
{
    pub fn new(user_id: Uuid) -> Self
    {
        Self
        {
            user_id: user_id,
            friend_ids: None,
            blocked_ids: None,
        }
    }
}