use crate::model::user::User;

pub trait Parent
{
    fn can_read_channel(&self, user: &User) -> bool;
    fn can_write_channel(&self, user: &User) -> bool;
}