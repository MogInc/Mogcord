use std::collections::HashMap;
use bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::model::channel_parent::Role;

//_id gets an ObjectId signed and will most likely do some voodoo to retrieve a chat
#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::pub_underscore_fields)]
#[allow(clippy::used_underscore_binding)]
pub struct MongolServer
{
    _id: Uuid,
    name: String,
    owner_id: Uuid,
    user_ids: Vec<Uuid>,
    channel_ids: Vec<Uuid>,
    //key is user id
    roles: HashMap<Uuid, Vec<Role>>,
}