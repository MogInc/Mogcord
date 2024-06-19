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

impl MongolUser
{
    pub async fn convert_to_db(user : &User) -> Result<MongolUser, MongolError>
    {
        match Uuid::parse_str(&user.uuid)
        {
            Ok(parsed_id) => Ok(
                MongolUser
                {
                    _id: parsed_id.clone(),
                    name: user.name.clone(),
                    mail: user.mail.clone(),
                }
            ),
            Err(_) => Err(MongolError::FailedUserParsing)
        }
    }

    pub fn convert_to_domain(self) -> User
    {
        User::convert(self._id.to_string(), self.name, self.mail)
    }
}