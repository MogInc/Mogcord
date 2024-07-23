use bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::db::MongolChannel;

//_id gets an ObjectId signed and will most likely do some voodoo to retrieve a chat
#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::pub_underscore_fields)]
#[allow(clippy::used_underscore_binding)]
pub struct MongolGroup
{
    pub _id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub user_ids: Vec<Uuid>,
    pub channel: MongolChannel,
}