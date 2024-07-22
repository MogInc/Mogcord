pub trait Parent
{
    fn can_read_channel(&self, user_id: &str) -> bool;
    fn can_write_channel(&self, user_id: &str) -> bool;
}