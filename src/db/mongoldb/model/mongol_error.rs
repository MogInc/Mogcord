use strum_macros::Display;

#[derive(Debug, Display)]
pub enum MongolError
{
    InvalidUUID,
    FailedUserParsing,
    FailedChatParsing,
    FailedDateParsing,
}