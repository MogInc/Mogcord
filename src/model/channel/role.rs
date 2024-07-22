use serde::{Deserialize, Serialize};

use super::Rights;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Role
{
    name: String,
    rights: Vec<Rights>,
}

