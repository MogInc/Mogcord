use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User
{
    pub user_uuid: String,
    pub user_name: String,
    pub user_mail: String,
}

impl User
{
    pub fn convert(uuid: String, name: String, mail: String) -> Self
    {
        User
        {
            user_uuid: uuid,
            user_name: name,
            user_mail: mail
        }
    }

    pub fn new(name: String, mail: String) -> Self
    {
        User
        {
            user_uuid: Uuid::new_v4().to_string(),
            user_name: name,
            user_mail: mail
        }
    }
}
