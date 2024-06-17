use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use crate::model::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolUser
{
    pub _id : ObjectId,
    pub uuid: String,
    pub name: String,
    pub mail: String,
}

impl MongolUser
{
    pub fn convert_to_db(user : &User) -> MongolUser
    {
        MongolUser
        {
            _id: ObjectId::new(),
            uuid: user.uuid.clone(),
            name: user.name.clone(),
            mail: user.mail.clone()
        }
    }

    pub fn convert_to_domain(self) -> User
    {
        User::convert(self.uuid, self.name, self.mail)
    }
}