use std::fmt;

#[derive(Debug)]
pub enum MongolError
{
    InvalidUUID,
    FailedUserParsing,
    FailedChatParsing,
}

impl fmt::Display for MongolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MongolError::InvalidUUID => write!(f, "Invalid UUID"),
            MongolError::FailedUserParsing => write!(f, "Couldn't parse user"),
            MongolError::FailedChatParsing => write!(f, "Couldn't parse chat"),
        }
    }
}