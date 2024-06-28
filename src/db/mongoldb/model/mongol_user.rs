use mongodb::bson::Uuid;
use serde::{Serialize, Deserialize};
use crate::{db::mongoldb::mongol_helper, model::user::User};

use super::MongolError;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolUser
{
    pub _id: Uuid,
    pub name: String,
    pub mail: String,
}

impl TryFrom<&User> for MongolUser
{
    type Error = MongolError;

    fn try_from(value: &User) -> Result<Self, Self::Error> 
    {
        let user_id = mongol_helper::convert_domain_id_to_mongol(&value.id)?;

        Ok(
            Self
            {
                _id: user_id,
                name: value.name.clone(),
                mail: value.mail.clone(),
            }
        )
    }
}

pub struct MongolUserVec(pub Vec<MongolUser>);

impl TryFrom<&Vec<User>> for MongolUserVec 
{
    type Error = MongolError;

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
        User::convert(value._id.to_string(), value.name.clone(), value.mail.clone())
    }
}