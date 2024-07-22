use crate::model::error;

pub trait Parent
{
    fn can_read(&self, user_id: &str, channel_id: Option<&str>) -> Result<bool, error::Server>;
    fn can_write(&self, user_id: &str, channel_id: Option<&str>) -> Result<bool, error::Server>;
}