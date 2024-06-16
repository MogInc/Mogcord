use serde::Serialize;
use uuid::Uuid;
use strum_macros::{EnumString, Display};

#[derive(Serialize)]
pub struct User
{
    pub user_uuid: String,
    pub user_name: String,
    pub user_mail: String,
}

impl User
{
    pub fn new(name: String, mail: String) -> User
    {
        User
        {
            user_uuid: Uuid::new_v4().to_string(),
            user_name: name,
            user_mail: mail
        }
    }
}