use crate::model::user::User;

pub trait Parent
{
    fn can_read(&self, user: &User) -> bool;
    fn can_write(&self, user: &User) -> bool;
}