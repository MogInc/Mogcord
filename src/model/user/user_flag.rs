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

#[cfg(test)]
mod tests 
{
    use std::str::FromStr;

    use chrono::Utc;

    use super::UserFlag;

    
    #[test]
    fn test_from_str_none_all_lowercase_is_valid() 
    {
        let enum_value = "none";
        let _result = UserFlag::from_str(&enum_value).unwrap();

        assert!(matches!(UserFlag::None, _result));
    }

    #[test]
    fn test_from_str_none_all_uppercase_is_valid() 
    {
        let enum_value = "NONE";
        let _result = UserFlag::from_str(&enum_value).unwrap();

        assert!(matches!(UserFlag::None, _result));
    }

    #[test]
    fn test_from_str_disabled_all_lowercase_is_valid() 
    {
        let enum_value = "disabled";
        let _result = UserFlag::from_str(&enum_value).unwrap();

        assert!(matches!(UserFlag::Disabled, _result));
    }

    #[test]
    fn test_from_str_disabled_all_uppercase_is_valid() {
        let enum_value = "DISABLED";
        let _result = UserFlag::from_str(&enum_value).unwrap();

        assert!(matches!(UserFlag::Disabled, _result));
    }

    #[test]
    fn test_from_str_deleted_all_lowercase_is_valid() 
    {
        let utc = Utc::now();
        let utc_str = utc.to_string();

        let mut enum_value = "deleted|".to_owned();
        enum_value.push_str(&utc_str);
        let _result = UserFlag::from_str(&enum_value).unwrap();

        assert!(matches!(UserFlag::Disabled, _result));
    }

    #[test]
    fn test_from_str_deleted_all_uppercase_is_valid() 
    {
        let utc = Utc::now();
        let utc_str = utc.to_string();

        let mut enum_value = "DELETED|".to_owned();
        enum_value.push_str(&utc_str);
        let _result = UserFlag::from_str(&enum_value).unwrap();

        assert!(matches!(UserFlag::Disabled, _result));
    }

    #[test]
    fn test_from_str_banned_all_lowercase_is_valid() 
    {
        let utc = Utc::now();
        let utc_str = utc.to_string();

        let mut enum_value = "banned|".to_owned();
        enum_value.push_str(&utc_str);
        let _result = UserFlag::from_str(&enum_value).unwrap();

        assert!(matches!(UserFlag::Disabled, _result));
    }

    #[test]
    fn test_from_str_banned_all_uppercase_is_valid() 
    {
        let utc = Utc::now();
        let utc_str = utc.to_string();

        let mut enum_value = "BANNED|".to_owned();
        enum_value.push_str(&utc_str);
        let _result = UserFlag::from_str(&enum_value).unwrap();

        assert!(matches!(UserFlag::Disabled, _result));
    }

    #[test]
    fn test_from_str_admin_all_lowercase_is_valid() 
    {
        let enum_value = "admin";
        let _result = UserFlag::from_str(&enum_value).unwrap();

        assert!(matches!(UserFlag::Disabled, _result));
    }

    #[test]
    fn test_from_str_admin_all_uppercase_is_valid() {
        let enum_value = "ADMIN";
        let _result = UserFlag::from_str(&enum_value).unwrap();

        assert!(matches!(UserFlag::Disabled, _result));
    }

    #[test]
    fn test_from_str_owner_all_lowercase_is_valid() 
    {
        let enum_value = "owner";
        let _result = UserFlag::from_str(&enum_value).unwrap();

        assert!(matches!(UserFlag::Disabled, _result));
    }

    #[test]
    fn test_from_str_owned_all_uppercase_is_valid() {
        let enum_value = "OWNER";
        let _result = UserFlag::from_str(&enum_value).unwrap();

        assert!(matches!(UserFlag::Disabled, _result));
    }
}