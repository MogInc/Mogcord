use crate::model::{channel_parent::Roles, error};
use super::Channel;

pub trait Parent
{
    fn get_channel(&self, channel_id_option: Option<&str>) -> Result<&Channel, error::Server>;
    fn get_user_roles(&self, user_id: &str) -> Option<&Roles>;
    fn can_read(&self, user_id: &str, channel_id_option: Option<&str>) -> Result<bool, error::Server>;
    fn can_write(&self, user_id: &str, channel_id_option: Option<&str>) -> Result<bool, error::Server>;
}