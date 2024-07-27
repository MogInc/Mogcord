use crate::model::error;
use super::Channel;

pub trait Parent
{
    fn get_channel<'input, 'stack>(&'input self, channel_id_option: Option<&'input str>) -> Result<&'input Channel, error::Server<'stack>>;
    fn get_user_roles(&self, user_id: &str) -> Option<&Vec<String>>;
    fn can_read<'input, 'stack>(&'input self, user_id: &'input str, channel_id_option: Option<&'input str>) -> Result<bool, error::Server<'stack>>;
    fn can_write<'input, 'stack>(&'input self, user_id: &'input str, channel_id_option: Option<&'input str>) -> Result<bool, error::Server<'stack>>;
}