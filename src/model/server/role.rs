use serde::{Deserialize, Serialize};

use crate::model::user::User;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Role
{
    name: String,
    rights: Vec<User>,
}

