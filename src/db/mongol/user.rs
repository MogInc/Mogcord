mod repository;


use bson::Bson;
use mongodb::bson::Uuid;
use serde::{Serialize, Deserialize};

use crate::model::{error, user::{self, User}};

use super::helper::{self, as_string};

#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::pub_underscore_fields)]
#[allow(clippy::used_underscore_binding)]
pub struct MongolUser
{
    pub _id: Uuid,
    pub username: String,
    pub mail: String,
    pub hashed_password: String,
    #[serde(serialize_with = "as_string")]
    pub flag: user::Flag,
}

impl TryFrom<&User> for MongolUser
{
    type Error = error::Server;

    fn try_from(value: &User) -> Result<Self, Self::Error> 
    {
        let user_id = helper::convert_domain_id_to_mongol(&value.id)?;

        Ok(
            Self
            {
                _id: user_id,
                username: value.username.clone(),
                mail: value.mail.clone(),
                hashed_password: value.hashed_password.clone(),
                flag: value.flag.clone(),
            }
        )
    }
}

pub struct MongolUserVec(pub Vec<MongolUser>);

impl TryFrom<&Vec<User>> for MongolUserVec 
{
    type Error = error::Server;

    fn try_from(value: &Vec<User>) -> Result<Self, Self::Error> 
    {
        let mut db_users = Vec::new();

        for user in value
        {
            db_users.push(MongolUser::try_from(user)?);
        }

        Ok(MongolUserVec(db_users))
    }
}

impl From<&MongolUser> for User
{
    fn from(value: &MongolUser) -> Self 
    {
        User::convert(
            value._id.to_string(),
            value.username.clone(), 
            value.mail.clone(),
            value.hashed_password.clone(),
            value.flag.clone(),
        )
    }
}

impl From<user::Flag> for Bson 
{
    fn from(user_flag: user::Flag) -> Bson 
    {
        Bson::String(user_flag.to_string())
    }
}