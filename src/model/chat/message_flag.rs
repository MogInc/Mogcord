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

#[derive(Debug)]
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
        let parts: Vec<&str> = input
                                .splitn(2,'|')
                                .map(|x| x.trim())
                                .collect();
                            
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

#[cfg(test)]
mod tests
{
    use std::str::FromStr;

    use chrono::Utc;

    use crate::model::chat::MessageFlag;

    #[test]
    fn test_from_str_none_all_lowercase_is_valid() 
    {
        let enum_value = "none";
        let result = MessageFlag::from_str(&enum_value).unwrap();

        assert_matches!(result, MessageFlag::None);
    }

    #[test]
    fn test_from_str_none_all_uppercase_is_valid() 
    {
        let enum_value = "NONE";
        let result = MessageFlag::from_str(&enum_value).unwrap();

        assert_matches!(result, MessageFlag::None);
    }

    #[test]
    fn test_from_str_edited_all_lowercase_is_valid() 
    {
        let utc = Utc::now();
        let utc_string = utc.to_rfc3339();

        let mut enum_value = "edited|".to_owned();
        enum_value.push_str(&utc_string);

        let result = MessageFlag::from_str(&enum_value).unwrap();

        assert_matches!(result, MessageFlag::Edited { .. });
        match result
        {
            MessageFlag::Edited { date } => assert_eq!(utc, date) ,
            _ => assert!(false, "Failed"),
        }
    }

    #[test]
    fn test_from_str_edited_all_uppercase_is_valid() 
    {
        let utc = Utc::now();
        let utc_string = utc.to_rfc3339();

        let mut enum_value = "EDITED|".to_owned();
        enum_value.push_str(&utc_string);

        let result = MessageFlag::from_str(&enum_value).unwrap();

        assert_matches!(result, MessageFlag::Edited { .. });
        match result
        {
            MessageFlag::Edited { date } => assert_eq!(utc, date) ,
            _ => assert!(false, "Failed"),
        }
    }

    #[test]
    fn test_from_str_deleted_all_lowercase_is_valid() 
    {
        let utc = Utc::now();
        let utc_string = utc.to_rfc3339();

        let mut enum_value = "deleted|".to_owned();
        enum_value.push_str(&utc_string);

        let result = MessageFlag::from_str(&enum_value).unwrap();

        assert_matches!(result, MessageFlag::Deleted { .. });
        match result
        {
            MessageFlag::Deleted { date } => assert_eq!(utc, date) ,
            _ => assert!(false, "Failed"),
        }
    }

    #[test]
    fn test_from_str_deleted_all_uppercase_is_valid() 
    {
        let utc = Utc::now();
        let utc_string = utc.to_rfc3339();

        let mut enum_value = "DELETED|".to_owned();
        enum_value.push_str(&utc_string);

        let result = MessageFlag::from_str(&enum_value).unwrap();

        assert_matches!(result, MessageFlag::Deleted { .. });
        match result
        {
            MessageFlag::Deleted { date } => assert_eq!(utc, date) ,
            _ => assert!(false, "Failed"),
        }
    }
}