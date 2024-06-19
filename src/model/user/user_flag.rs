use std::str::FromStr;
use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
        let parts: Vec<&str> = input
                                .splitn(2,'|')
                                .map(|x| x.trim())
                                .collect();
                            
        match parts[0].to_lowercase().as_str() 
        {
            "none" => Ok(UserFlag::None),
            "disabled" => Ok(UserFlag::Disabled),
            "deleted" => 
            {
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
            "banned" => 
            {
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

    use crate::model::user::UserFlagParseError;
    use super::UserFlag;

    macro_rules! from_str_base_tests_valid
    {
        ($($name:ident: $value:expr,)*) => 
        {
        $(
            #[test]
            fn $name() 
            {
                let (input, expected) = $value;
                let result = UserFlag::from_str(input).unwrap();
                assert_eq!(result, expected);
            }
        )*
        }
    }

    from_str_base_tests_valid!
    {
        test_from_str_none_all_lowercase_is_valid:("none", UserFlag::None),
        test_from_str_none_all_uppercase_is_valid:("NONE", UserFlag::None),
        test_from_str_disabled_all_lowercase_is_valid:("disabled", UserFlag::None),
        test_from_str_disabled_all_uppercase_is_valid:("DISABLED", UserFlag::None),
        test_from_str_admin_all_lowercase_is_valid:("admin", UserFlag::None),
        test_from_str_admin_all_uppercase_is_valid:("ADMIN", UserFlag::None),
        test_from_str_owner_all_lowercase_is_valid:("owner", UserFlag::None),
        test_from_str_owner_all_uppercase_is_valid:("OWNER", UserFlag::None),
    }

    macro_rules! from_str_base_tests_invalid
    {
        ($($name:ident: $value:expr,)*) => 
        {
        $(
            #[test]
            fn $name() 
            {
                let (input, expected) = $value;
                let result = UserFlag::from_str(input).unwrap_err();
                assert_eq!(result, expected);
            }
        )*
        }
    }

    from_str_base_tests_invalid! 
    {
        test_from_str_is_invalid:("AAAaaa", UserFlagParseError::InvalidFormat),
        test_from_str_edited_invalid_format_is_invalid:("edited", UserFlagParseError::InvalidFormat),
        test_from_str_edited_invalid_separator_is_invalid:("edited+utc_time", UserFlagParseError::InvalidFormat),
        test_from_str_edited_invalid_date_is_invalid:("edited|utc_time", UserFlagParseError::InvalidDate),
        test_from_str_deleted_invalid_format_is_invalid:("deleted", UserFlagParseError::InvalidFormat),
        test_from_str_deleted_invalid_separator_is_invalid:("deleted+utc_time", UserFlagParseError::InvalidFormat),
        test_from_str_deleted_invalid_date_is_invalid:("deleted|utc_time", UserFlagParseError::InvalidDate),
    }


    #[test]
    fn test_from_str_deleted_all_lowercase_is_valid() 
    {
        let utc = Utc::now();
        let utc_string = utc.to_rfc3339();

        let mut enum_value = "deleted|".to_owned();
        enum_value.push_str(&utc_string);
        let result = UserFlag::from_str(&enum_value).unwrap();

        assert_matches!(result, UserFlag::Deleted { .. });
        match result
        {
            UserFlag::Deleted{ date } => assert_eq!(date, utc),
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
        let result = UserFlag::from_str(&enum_value).unwrap();

        assert_matches!(result, UserFlag::Deleted{ .. });
        match result
        {
            UserFlag::Deleted{ date } => assert_eq!(date, utc),
            _ => assert!(false, "Failed"),
        }
    }


    #[test]
    fn test_from_str_banned_all_lowercase_is_valid() 
    {
        let utc = Utc::now();
        let utc_string = utc.to_string();

        let mut enum_value = "banned|".to_owned();
        enum_value.push_str(&utc_string);
        let result = UserFlag::from_str(&enum_value).unwrap();

        assert_matches!(result, UserFlag::Banned{ .. });
        match result
        {
            UserFlag::Banned{ date } => assert_eq!(date, utc),
            _ => assert!(false, "Failed"),
        }
    }

    #[test]
    fn test_from_str_banned_all_uppercase_is_valid() 
    {
        let utc = Utc::now();
        let utc_string = utc.to_string();

        let mut enum_value = "BANNED|".to_owned();
        enum_value.push_str(&utc_string);
        let result = UserFlag::from_str(&enum_value).unwrap();

        assert_matches!(result, UserFlag::Banned{ .. });
        match result
        {
            UserFlag::Banned{ date } => assert_eq!(date, utc),
            _ => assert!(false, "Failed"),
        }
    }

    #[test]
    fn test_from_str_banned_invalid_format_is_invalid() 
    {
        let enum_value = "banned";
        let result = UserFlag::from_str(&enum_value).unwrap_err();

        assert_matches!(result, UserFlagParseError::InvalidFormat);
    }

    #[test]
    fn test_from_str_banned_invalid_date_is_invalid() 
    {
        let enum_value = "banned|24-10-2024";
        let result = UserFlag::from_str(&enum_value).unwrap_err();
        
        assert_matches!(result, UserFlagParseError::InvalidDate);
    }
}