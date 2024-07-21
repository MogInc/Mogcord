use serde::{Deserialize, Serialize};
use uuid::Uuid;



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Channel
{
    pub id: String,
    pub name: Option<String>,
}

impl Channel
{
    #[must_use]
    pub fn new(name: Option<String>) -> Self
    {
        let name_sanitized = name.map(|name| name.trim().to_owned());

        Self
        {
            id: Uuid::now_v7().to_string(),
            name: name_sanitized,
        }
    }
}