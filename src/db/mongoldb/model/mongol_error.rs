use strum_macros::Display;

#[derive(Debug, Display)]
pub enum MongolError
{
    InvalidID,
    FailedUserParsing,
    FailedChatParsing,
    FailedDateParsing,
}