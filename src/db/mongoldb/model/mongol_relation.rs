use bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolRelation
{
    user_id: Uuid,
    friend_ids: Vec<Uuid>,
    pending_incoming_friend_ids: Vec<Uuid>,
    pending_outgoing_friend_ids: Vec<Uuid>,
    blocked_ids: Vec<Uuid>,
}

//todo: add friend count and blocked count when Count gets expensive
//i assume its the same as n + 1 problem

impl MongolRelation
{
    pub fn new(user_id: Uuid) -> Self
    {
        Self
        {
            user_id: user_id,
            friend_ids: Vec::new(),
            pending_incoming_friend_ids: Vec::new(),
            pending_outgoing_friend_ids: Vec::new(),
            blocked_ids: Vec::new(),
        }
    }
}