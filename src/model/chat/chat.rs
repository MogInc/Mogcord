use crate::model::user::User;

pub enum ChatType
{
    Private,
    Group,
    Server,
}

pub struct Chat
{
    pub uuid: String,
    pub name: String,
    pub r#type: ChatType,
    pub owner: User,
    pub members: Vec<User>,
}