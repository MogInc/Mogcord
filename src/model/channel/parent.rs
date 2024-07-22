pub trait Parent
{
    fn can_read(&self, user_id: &str, channel_id: Option<&str>) -> bool;
    fn can_write(&self, user_id: &str, channel_id: Option<&str>) -> bool;
}