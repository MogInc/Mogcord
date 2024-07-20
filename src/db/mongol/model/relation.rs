use bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolRelation
{
    pub user_id: Uuid,
    pub friend_ids: Vec<Uuid>,
    pub pending_incoming_friend_ids: Vec<Uuid>,
    pub pending_outgoing_friend_ids: Vec<Uuid>,
    pub blocked_ids: Vec<Uuid>,
}

//todo: add friend count and blocked count when Count gets expensive
//i assume its the same as n + 1 problem

impl MongolRelation
{
    #[must_use]
    pub fn new(user_id: Uuid) -> Self
    {
        Self
        {
            user_id,
            friend_ids: Vec::new(),
            pending_incoming_friend_ids: Vec::new(),
            pending_outgoing_friend_ids: Vec::new(),
            blocked_ids: Vec::new(),
        }
    }
}