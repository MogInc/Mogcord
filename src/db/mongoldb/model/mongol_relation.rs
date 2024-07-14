use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolRelation
{
    user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    friend_ids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blocked_ids: Option<String>,
}