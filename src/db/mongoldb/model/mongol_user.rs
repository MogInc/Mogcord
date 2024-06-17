use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use crate::model::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolUser
{
    pub _id : ObjectId,
    pub user_uuid: String,
    pub user_name: String,
    pub user_mail: String,
}

impl MongolUser
{
    pub fn convert_to_db(user : &User) -> MongolUser
    {
        MongolUser
        {
            _id: ObjectId::new(),
            user_uuid: user.user_uuid.clone(),
            user_name: user.user_name.clone(),
            user_mail: user.user_mail.clone()
        }
    }

    pub fn convert_to_domain(self) -> User
    {
        User::convert(self.user_uuid, self.user_name, self.user_mail)
    }
}