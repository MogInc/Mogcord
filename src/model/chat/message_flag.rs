use std::{fmt, str::FromStr};
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum MessageFlagParseError 
{
    InvalidFormat,
    InvalidDate,
}

impl fmt::Display for MessageFlagParseError 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        match *self 
        {
            MessageFlagParseError::InvalidFormat => write!(f, "Invalid format"),
            MessageFlagParseError::InvalidDate => write!(f, "Invalid date"),
        }
    }
}

impl std::error::Error for MessageFlagParseError {}

pub enum MessageFlag
{
    None,
    Edited { date: DateTime<Utc> },
    Deleted { date: DateTime<Utc> },
}

impl FromStr for MessageFlag 
{
    type Err = MessageFlagParseError;

    fn from_str(input: &str) -> Result<MessageFlag, Self::Err> 
    {
        let parts: Vec<&str> = input.splitn(2, '|').collect();
        match parts[0].to_lowercase().as_str() 
        {
            "none" => Ok(MessageFlag::None),
            "edited" => 
            {
                if parts.len() == 2 
                {
                    parts[1].parse::<DateTime<Utc>>()
                        .map(|date| MessageFlag::Edited{ date })
                        .map_err(|_| MessageFlagParseError::InvalidDate)
                } else 
                {
                    Err(MessageFlagParseError::InvalidFormat)
                }
            }
            "deleted" => 
            {
                if parts.len() == 2 
                {
                    parts[1].parse::<DateTime<Utc>>()
                        .map(|date| MessageFlag::Deleted{ date })
                        .map_err(|_| MessageFlagParseError::InvalidDate)
                } else 
                {
                    Err(MessageFlagParseError::InvalidFormat)
                }
            }
            _ => Err(MessageFlagParseError::InvalidFormat),
        }
    }
}