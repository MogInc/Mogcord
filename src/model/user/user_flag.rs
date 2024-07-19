use std::str::FromStr;
use chrono::{DateTime, Utc};
use serde::{de::{self, Visitor}, Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize, Eq, Hash)]
pub enum UserFlag 
{
    None,
    Disabled,
    Deleted { date: DateTime<Utc> },
    Banned { date: DateTime<Utc> },
    Admin,
    Owner,
}

impl UserFlag
{
    pub fn is_admin_or_owner(&self) -> bool
    {
        match &self
        {
            Self::Admin | Self::Owner => true,
            _ => false,
        }
    }

    pub fn is_allowed_on_mogcord(&self) -> bool
    {
        match &self
        {
            Self::None | Self::Admin | Self::Owner => true,
            _ => false,
        }
    }
}

impl fmt::Display for UserFlag 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
        match self
        {
            Self::None => write!(f, "none"),
            Self::Disabled => write!(f, "disabled"),
            Self::Banned { date } => write!(f, "banned|{}", date),
            Self::Deleted { date } => write!(f, "deleted|{}", date),
            Self::Admin => write!(f, "admin"),
            Self::Owner => write!(f, "owner"),
        }
    }
}

impl<'de> Deserialize<'de> for UserFlag
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> 
    {
        struct UserFlagVisitor;

        impl<'de> Visitor<'de> for UserFlagVisitor
        {
            type Value = UserFlag;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result 
            {
                formatter.write_str("data")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where E: serde::de::Error, 
            {
                UserFlag::from_str(v)
                    .map_err(|_| de::Error::unknown_field(v, FIELDS))
            }
        }

        const FIELDS: &[&str] = &["none", "disabled", "deleted", "banned", "admin", "owner"];
        
        deserializer.deserialize_identifier(UserFlagVisitor)
    }
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
                    parts[1]
                    .parse::<DateTime<Utc>>()
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
                    parts[1]
                    .parse::<DateTime<Utc>>()
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
        test_from_str_none_all_lowercase_with_whitespace_is_valid: (" none ", UserFlag::None),
        test_from_str_none_all_lowercase_with_lf_is_valid: ("\nnone\n", UserFlag::None),
        test_from_str_none_all_lowercase_with_cr_is_valid: ("\rnone\r", UserFlag::None),
        test_from_str_none_all_lowercase_with_crlf_is_valid: ("\r\nnone\r\n", UserFlag::None),
        test_from_str_none_all_uppercase_is_valid: ("NONE", UserFlag::None),
        test_from_str_none_all_uppercase_with_whitespace_is_valid: (" NONE ", UserFlag::None),
        test_from_str_none_all_uppercase_with_lf_is_valid: ("\nNONE\n", UserFlag::None),
        test_from_str_none_all_uppercase_with_cr_is_valid: ("\rNONE\r", UserFlag::None),
        test_from_str_none_all_uppercase_with_crlf_is_valid: ("\r\nNONE\r\n", UserFlag::None),
        test_from_str_none_variant_casing_is_valid: ("nONe", UserFlag::None),
        test_from_str_none_variant_casing_with_whitespace_is_valid: (" nONe ", UserFlag::None),
        test_from_str_none_variant_casing_with_lf_is_valid: ("\nnONe\n", UserFlag::None),
        test_from_str_none_variant_casing_with_cr_is_valid: ("\rnONe\r", UserFlag::None),
        test_from_str_none_variant_casing_with_crlf_is_valid: ("\r\nnONe\r\n", UserFlag::None),
        test_from_str_disabled_all_lowercase_is_valid:("disabled", UserFlag::Disabled),
        test_from_str_disabled_all_lowercase_with_whitespace_is_valid: (" disabled ", UserFlag::Disabled),
        test_from_str_disabled_all_lowercase_with_lf_is_valid: ("\ndisabled\n", UserFlag::Disabled),
        test_from_str_disabled_all_lowercase_with_cr_is_valid: ("\rdisabled\r", UserFlag::Disabled),
        test_from_str_disabled_all_lowercase_with_crlf_is_valid: ("\r\ndisabled\r\n", UserFlag::Disabled),
        test_from_str_disabled_all_uppercase_is_valid:("DISABLED", UserFlag::Disabled),
        test_from_str_disabled_all_uppercase_with_whitespace_is_valid: (" DISABLED ", UserFlag::Disabled),
        test_from_str_disabled_all_uppercase_with_lf_is_valid: ("\nDISABLED\n", UserFlag::Disabled),
        test_from_str_disabled_all_uppercase_with_cr_is_valid: ("\rDISABLED\r", UserFlag::Disabled),
        test_from_str_disabled_all_uppercase_with_crlf_is_valid: ("\r\nDISABLED\r\n", UserFlag::Disabled),
        test_from_str_disabled_variant_casing_is_valid:("disaBLEd", UserFlag::Disabled),
        test_from_str_disabled_variant_casing_with_whitespace_is_valid: (" disaBLEd ", UserFlag::Disabled),
        test_from_str_disabled_variant_casing_with_lf_is_valid: ("\ndisaBLEd\n", UserFlag::Disabled),
        test_from_str_disabled_variant_casing_with_cr_is_valid: ("\rdisaBLEd\r", UserFlag::Disabled),
        test_from_str_disabled_variant_casing_with_crlf_is_valid: ("\r\ndisaBLEd\r\n", UserFlag::Disabled),
        test_from_str_admin_all_lowercase_is_valid:("admin", UserFlag::Admin),
        test_from_str_admin_all_lowercase_with_whitespace_is_valid: (" admin ", UserFlag::Admin),
        test_from_str_admin_all_lowercase_with_lf_is_valid: ("\nadmin\n", UserFlag::Admin),
        test_from_str_admin_all_lowercase_with_cr_is_valid: ("\radmin\r", UserFlag::Admin),
        test_from_str_admin_all_lowercase_with_crlf_is_valid: ("\r\nadmin\r\n", UserFlag::Admin),
        test_from_str_admin_all_uppercase_is_valid:("ADMIN", UserFlag::Admin),
        test_from_str_admin_all_uppercase_with_whitespace_is_valid: (" ADMIN ", UserFlag::Admin),
        test_from_str_admin_all_uppercase_with_lf_is_valid: ("\nADMIN\n", UserFlag::Admin),
        test_from_str_admin_all_uppercase_with_cr_is_valid: ("\rADMIN\r", UserFlag::Admin),
        test_from_str_admin_all_uppercase_with_crlf_is_valid: ("\r\nADMIN\r\n", UserFlag::Admin),
        test_from_str_admin_variant_casing_is_valid:("adMIn", UserFlag::Admin),
        test_from_str_admin_variant_casing_with_whitespace_is_valid: (" adMIn ", UserFlag::Admin),
        test_from_str_admin_variant_casing_with_lf_is_valid: ("\nadMIn\n", UserFlag::Admin),
        test_from_str_admin_variant_casing_with_cr_is_valid: ("\radMIn\r", UserFlag::Admin),
        test_from_str_admin_variant_casing_with_crlf_is_valid: ("\r\nadMIn\r\n", UserFlag::Admin),
        test_from_str_owner_all_lowercase_is_valid:("owner", UserFlag::Owner),
        test_from_str_owner_all_lowercase_with_whitespace_is_valid: (" owner ", UserFlag::Owner),
        test_from_str_owner_all_lowercase_with_lf_is_valid: ("\nowner\n", UserFlag::Owner),
        test_from_str_owner_all_lowercase_with_cr_is_valid: ("\rowner\r", UserFlag::Owner),
        test_from_str_owner_all_lowercase_with_crlf_is_valid: ("\r\nowner\r\n", UserFlag::Owner),
        test_from_str_owner_all_uppercase_is_valid:("OWNER", UserFlag::Owner),
        test_from_str_owner_all_uppercase_with_whitespace_is_valid: (" OWNER ", UserFlag::Owner),
        test_from_str_owner_all_uppercase_with_lf_is_valid: ("\nOWNER\n", UserFlag::Owner),
        test_from_str_owner_all_uppercase_with_cr_is_valid: ("\rOWNER\r", UserFlag::Owner),
        test_from_str_owner_all_uppercase_with_crlf_is_valid: ("\r\nOWNER\r\n", UserFlag::Owner),
        test_from_str_owner_variant_casing_is_valid:("owNER", UserFlag::Owner),
        test_from_str_owner_variant_casing_with_whitespace_is_valid: (" owNER ", UserFlag::Owner),
        test_from_str_owner_variant_casing_with_lf_is_valid: ("\nowNER\n", UserFlag::Owner),
        test_from_str_owner_variant_casing_with_cr_is_valid: ("\rowNER\r", UserFlag::Owner),
        test_from_str_owner_variant_casing_with_crlf_is_valid: ("\r\nowNER\r\n", UserFlag::Owner),
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
        test_from_str_edited_invalid_format_is_invalid:("deleted", UserFlagParseError::InvalidFormat),
        test_from_str_edited_invalid_separator_is_invalid:("deleted+utc_time", UserFlagParseError::InvalidFormat),
        test_from_str_edited_invalid_date_is_invalid:("deleted|utc_time", UserFlagParseError::InvalidDate),
        test_from_str_deleted_invalid_format_is_invalid:("banned", UserFlagParseError::InvalidFormat),
        test_from_str_deleted_invalid_separator_is_invalid:("banned+utc_time", UserFlagParseError::InvalidFormat),
        test_from_str_deleted_invalid_date_is_invalid:("banned|utc_time", UserFlagParseError::InvalidDate),
    }

    macro_rules! from_str_date_tests_valid 
    {
        ($($name:ident: $value:expr,)*) => 
        {
        $(
            #[test]
            fn $name() 
            {
                let (input, utc, expected) = $value;

                let utc_string = utc.to_rfc3339();
        
                let mut enum_value = input.to_owned();
                enum_value.push_str(&utc_string);
        
                let result = UserFlag::from_str(&enum_value).unwrap();
        
                assert_eq!(result, expected);
            }
        )*
        }
    }

    from_str_date_tests_valid!
    {
        test_from_str_deleted_all_lowercase_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("deleted|", fixed_utc, UserFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_lowercase_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" deleted | ", fixed_utc, UserFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_lowercase_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\ndeleted\n|\n", fixed_utc, UserFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_lowercase_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rdeleted\r|\r", fixed_utc, UserFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_lowercase_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\ndeleted\r\n|\r\n", fixed_utc, UserFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_uppercase_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("DELETED|", fixed_utc, UserFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_uppercase_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" DELETED | ", fixed_utc, UserFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_uppercase_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rDELETED\n|\n", fixed_utc, UserFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_uppercase_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rDELETED\r|\r", fixed_utc, UserFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_uppercase_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\nDELETED\r\n|\r\n", fixed_utc, UserFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_variant_casing_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("DeLEted|", fixed_utc, UserFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_variant_casing_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" DeLEted | ", fixed_utc, UserFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_variant_casing_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rDeLEted\n|\n", fixed_utc, UserFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_variant_casing_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rDeLEted\r|\r", fixed_utc, UserFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_variant_casing_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\nDeLEted\r\n|\r\n", fixed_utc, UserFlag::Deleted { date: fixed_utc })
        },
        test_from_str_banned_all_lowercase_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("banned|", fixed_utc, UserFlag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_lowercase_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" banned | ", fixed_utc, UserFlag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_lowercase_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\nbanned\n|\n", fixed_utc, UserFlag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_lowercase_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rbanned\r|\r", fixed_utc, UserFlag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_lowercase_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\nbanned\r\n|\r\n", fixed_utc, UserFlag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_uppercase_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("BANNED|", fixed_utc, UserFlag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_uppercase_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" BANNED | ", fixed_utc, UserFlag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_uppercase_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rBANNED\n|\n", fixed_utc, UserFlag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_uppercase_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rBANNED\r|\r", fixed_utc, UserFlag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_uppercase_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\nBANNED\r\n|\r\n", fixed_utc, UserFlag::Banned { date: fixed_utc })
        },
        test_from_str_banned_variant_casing_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("baNNEd|", fixed_utc, UserFlag::Banned { date: fixed_utc })
        },
        test_from_str_banned_variant_casing_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" baNNEd | ", fixed_utc, UserFlag::Banned { date: fixed_utc })
        },
        test_from_str_banned_variant_casing_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rbaNNEd\n|\n", fixed_utc, UserFlag::Banned { date: fixed_utc })
        },
        test_from_str_banned_variant_casing_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rbaNNEd\r|\r", fixed_utc, UserFlag::Banned { date: fixed_utc })
        },
        test_from_str_banned_variant_casing_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\nbaNNEd\r\n|\r\n", fixed_utc, UserFlag::Banned { date: fixed_utc })
        },
    }
}