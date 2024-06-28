use mongodb::bson::Uuid;
use serde::{Serialize, Deserialize};
use crate::model::user::User;

use super::MongolError;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolUser
{
    pub _id: Uuid,
    pub name: String,
    pub mail: String,
}

impl TryFrom<User> for MongolUser
{
    type Error = MongolError;

    fn try_from(value: User) -> Result<Self, Self::Error> 
    {
        match Uuid::parse_str(&value.id)
        {
            Ok(_id) => Ok(
                MongolUser
                {
                    _id: _id.clone(),
                    name: value.name.clone(),
                    mail: value.mail.clone(),
                }
            ),
            Err(_) => Err(MongolError::FailedUserParsing)
        }
    }
}

impl From<MongolUser> for User
{
    fn from(value: MongolUser) -> Self {
        User::convert(value._id.to_string(), value.name, value.mail)
    }
}