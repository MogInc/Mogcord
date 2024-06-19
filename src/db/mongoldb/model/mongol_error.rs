use std::fmt;

#[derive(Debug)]
pub enum MongolError
{
    FailedUserParsing
}

impl fmt::Display for MongolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MongolError::FailedUserParsing => write!(f, "Couldn't parse user"),
        }
    }
}