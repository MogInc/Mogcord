use bson::Bson;
use mongodb::bson::Uuid;
use serde::{Serialize, Deserialize};
use crate::{db::mongoldb::{as_string, mongol_helper}, model::{misc::ServerError, user::{User, UserFlag}}};

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolUser
{
    pub _id: Uuid,
    pub username: String,
    pub mail: String,
    pub hashed_password: String,
    #[serde(serialize_with = "as_string")]
    pub flag: UserFlag,
}

impl TryFrom<&User> for MongolUser
{
    type Error = ServerError;

    fn try_from(value: &User) -> Result<Self, Self::Error> 
    {
        let user_id = mongol_helper::convert_domain_id_to_mongol(&value.id)?;

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
    type Error = ServerError;

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

impl From<UserFlag> for Bson 
{
    fn from(user_flag: UserFlag) -> Bson 
    {
        Bson::String(user_flag.to_string())
    }
}