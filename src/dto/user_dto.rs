use serde::Serialize;

use crate::model::user::User;

#[derive(Serialize)]
pub struct UserDTO
{
    pub uuid: String,
    pub name: String,
    pub mail: String,
}

impl UserDTO
{
    pub fn obj_to_dto(user: User) -> Self
    {
        Self
        {
            uuid: user.uuid,
            name: user.name,
            mail: user.mail,
        }
    }
    
    pub fn vec_to_dto(users: Vec<User>) -> Vec<Self>
    {
        let mut users_dto: Vec<UserDTO> = Vec::new();

        for user in users
        {
            users_dto.push(Self::obj_to_dto(user))
        }
        
        return users_dto;
    }
}