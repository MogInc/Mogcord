use serde::Serialize;

use crate::model::user::User;

use super::ObjectToDTO;

#[derive(Serialize)]
pub struct UserCreateResponse
{
    pub id: String,
    pub username: String,
    pub mail: String,
}

impl ObjectToDTO<User> for UserCreateResponse
{
    fn obj_to_dto(user: User) -> Self
    {
        Self
        {
            id: user.id,
            username: user.username,
            mail: user.mail,
        }
    }
}