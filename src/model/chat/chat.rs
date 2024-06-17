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
    pub name: Option<String>,
    pub r#type: ChatType,
    pub owners: Vec<User>,
    pub members: Option<Vec<User>>,
    pub buckets: Option<Vec<Bucket>>
}

pub struct Bucket
{

}

pub struct Message
{
    pub uuid: String,
    pub value: String,
    pub owner: User,
    pub chat: Chat,
    pub bucket: Bucket,
}