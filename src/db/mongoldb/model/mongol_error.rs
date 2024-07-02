use std::fmt;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum MongolError
{
    InvalidID { id: String },
    FailedUserParsing,
    FailedChatParsing,
    FailedDateParsing,
}

impl fmt::Display for MongolError 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
        write!(f, "{self:?}")
    }
}