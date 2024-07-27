use crate::model::error;
use super::Channel;

pub trait Parent
{
    fn get_channel<'input, 'err>(&'input self, channel_id_option: Option<&'input str>) -> Result<&'input Channel, error::Server<'err>>;
    fn get_user_roles(&self, user_id: &str) -> Option<&Vec<String>>;
    fn can_read<'input, 'err>(&'input self, user_id: &'input str, channel_id_option: Option<&'input str>) -> Result<bool, error::Server<'err>>;
    fn can_write<'input, 'err>(&'input self, user_id: &'input str, channel_id_option: Option<&'input str>) -> Result<bool, error::Server<'err>>;
}