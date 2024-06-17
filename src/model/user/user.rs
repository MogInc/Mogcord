use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User
{
    pub uuid: String,
    pub name: String,
    pub mail: String,
}

impl User
{
    pub fn convert(uuid: String, name: String, mail: String) -> Self
    {
        User
        {
            uuid,
            name,
            mail
        }
    }

    pub fn new(name: String, mail: String) -> Self
    {
        User
        {
            uuid: Uuid::new_v4().to_string(),
            name,
            mail
        }
    }
}
