use std::str::FromStr;
use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug)]
pub enum UserFlagParseError 
{
    InvalidFormat,
    InvalidDate,
}

impl fmt::Display for UserFlagParseError 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        match *self 
        {
            UserFlagParseError::InvalidFormat => write!(f, "Invalid format"),
            UserFlagParseError::InvalidDate => write!(f, "Invalid date"),
        }
    }
}

impl std::error::Error for UserFlagParseError {}

pub enum UserFlag 
{
    None,
    Disabled,
    Deleted { date: DateTime<Utc> },
    Banned { date: DateTime<Utc> },
    Admin,
    Owner,
}

impl FromStr for UserFlag 
{
    type Err = UserFlagParseError;

    fn from_str(input: &str) -> Result<UserFlag, Self::Err> 
    {
        let parts: Vec<&str> = input.splitn(2, '|').collect();
        match parts[0].to_lowercase().as_str() 
        {
            "none" => Ok(UserFlag::None),
            "disabled" => Ok(UserFlag::Disabled),
            "deleted" => {
                if parts.len() == 2 
                {
                    parts[1].parse::<DateTime<Utc>>()
                        .map(|date| UserFlag::Deleted { date })
                        .map_err(|_| UserFlagParseError::InvalidDate)
                } else 
                {
                    Err(UserFlagParseError::InvalidFormat)
                }
            }
            "banned" => {
                if parts.len() == 2 
                {
                    parts[1].parse::<DateTime<Utc>>()
                        .map(|date| UserFlag::Banned { date })
                        .map_err(|_| UserFlagParseError::InvalidDate)
                } else 
                {
                    Err(UserFlagParseError::InvalidFormat)
                }
            }
            "admin" => Ok(UserFlag::Admin),
            "owner" => Ok(UserFlag::Owner),
            _ => Err(UserFlagParseError::InvalidFormat),
        }
    }
}