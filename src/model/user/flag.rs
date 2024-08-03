use std::str::FromStr;
use chrono::{DateTime, Utc};
use serde::{de::{self, Visitor}, Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize, Eq, Hash)]
pub enum Flag 
{
    None,
    Disabled,
    Deleted { date: DateTime<Utc> },
    Banned { date: DateTime<Utc> },
    Admin,
    Owner,
}

impl Flag
{
    #[must_use]
    pub fn is_mogcord_admin_or_owner(&self) -> bool
    {
        matches!(self, Self::Admin | Self::Owner)
    }

    #[must_use]
    pub fn is_allowed_on_mogcord(&self) -> bool
    {
        matches!(self, Self::None | Self::Admin | Self::Owner)
    }
}

impl fmt::Display for Flag 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
        match self
        {
            Self::None => write!(f, "none"),
            Self::Disabled => write!(f, "disabled"),
            Self::Banned { date } => write!(f, "banned|{date}"),
            Self::Deleted { date } => write!(f, "deleted|{date}"),
            Self::Admin => write!(f, "admin"),
            Self::Owner => write!(f, "owner"),
        }
    }
}

impl<'de> Deserialize<'de> for Flag
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> 
    {
        struct UserFlagVisitor;

        impl<'de> Visitor<'de> for UserFlagVisitor
        {
            type Value = Flag;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result 
            {
                formatter.write_str("data")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where E: serde::de::Error, 
            {
                Flag::from_str(v)
                    .map_err(|_| de::Error::unknown_field(v, FIELDS))
            }
        }

        const FIELDS: &[&str] = &["none", "disabled", "deleted", "banned", "admin", "owner"];
        
        deserializer.deserialize_identifier(UserFlagVisitor)
    }
}

impl FromStr for Flag 
{
    type Err = UserFlagParseError;

    fn from_str(input: &str) -> Result<Flag, Self::Err> 
    {
        let parts: Vec<&str> = input
            .splitn(2,'|')
            .map(str::trim)
            .collect();
                            
        match parts[0].to_lowercase().as_str() 
        {
            "none" => Ok(Flag::None),
            "disabled" => Ok(Flag::Disabled),
            "deleted" => 
            {
                if parts.len() == 2 
                {
                    parts[1]
                    .parse::<DateTime<Utc>>()
                    .map(|date| Flag::Deleted { date })
                    .map_err(|_| UserFlagParseError::InvalidDate)
                } 
                else 
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
                    .map(|date| Flag::Banned { date })
                    .map_err(|_| UserFlagParseError::InvalidDate)
                } 
                else 
                {
                    Err(UserFlagParseError::InvalidFormat)
                }
            }
            "admin" => Ok(Flag::Admin),
            "owner" => Ok(Flag::Owner),
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

    use crate::model::user::flag::UserFlagParseError;

    use super::Flag;

    macro_rules! from_str_base_tests_valid
    {
        ($($name:ident: $value:expr,)*) => 
        {
        $(
            #[test]
            fn $name() 
            {
                let (input, expected) = $value;
                let result = Flag::from_str(input).unwrap();
                assert_eq!(result, expected);
            }
        )*
        }
    }

    from_str_base_tests_valid!
    {
        test_from_str_none_all_lowercase_is_valid:("none", Flag::None),
        test_from_str_none_all_lowercase_with_whitespace_is_valid: (" none ", Flag::None),
        test_from_str_none_all_lowercase_with_lf_is_valid: ("\nnone\n", Flag::None),
        test_from_str_none_all_lowercase_with_cr_is_valid: ("\rnone\r", Flag::None),
        test_from_str_none_all_lowercase_with_crlf_is_valid: ("\r\nnone\r\n", Flag::None),
        test_from_str_none_all_uppercase_is_valid: ("NONE", Flag::None),
        test_from_str_none_all_uppercase_with_whitespace_is_valid: (" NONE ", Flag::None),
        test_from_str_none_all_uppercase_with_lf_is_valid: ("\nNONE\n", Flag::None),
        test_from_str_none_all_uppercase_with_cr_is_valid: ("\rNONE\r", Flag::None),
        test_from_str_none_all_uppercase_with_crlf_is_valid: ("\r\nNONE\r\n", Flag::None),
        test_from_str_none_variant_casing_is_valid: ("nONe", Flag::None),
        test_from_str_none_variant_casing_with_whitespace_is_valid: (" nONe ", Flag::None),
        test_from_str_none_variant_casing_with_lf_is_valid: ("\nnONe\n", Flag::None),
        test_from_str_none_variant_casing_with_cr_is_valid: ("\rnONe\r", Flag::None),
        test_from_str_none_variant_casing_with_crlf_is_valid: ("\r\nnONe\r\n", Flag::None),
        test_from_str_disabled_all_lowercase_is_valid:("disabled", Flag::Disabled),
        test_from_str_disabled_all_lowercase_with_whitespace_is_valid: (" disabled ", Flag::Disabled),
        test_from_str_disabled_all_lowercase_with_lf_is_valid: ("\ndisabled\n", Flag::Disabled),
        test_from_str_disabled_all_lowercase_with_cr_is_valid: ("\rdisabled\r", Flag::Disabled),
        test_from_str_disabled_all_lowercase_with_crlf_is_valid: ("\r\ndisabled\r\n", Flag::Disabled),
        test_from_str_disabled_all_uppercase_is_valid:("DISABLED", Flag::Disabled),
        test_from_str_disabled_all_uppercase_with_whitespace_is_valid: (" DISABLED ", Flag::Disabled),
        test_from_str_disabled_all_uppercase_with_lf_is_valid: ("\nDISABLED\n", Flag::Disabled),
        test_from_str_disabled_all_uppercase_with_cr_is_valid: ("\rDISABLED\r", Flag::Disabled),
        test_from_str_disabled_all_uppercase_with_crlf_is_valid: ("\r\nDISABLED\r\n", Flag::Disabled),
        test_from_str_disabled_variant_casing_is_valid:("disaBLEd", Flag::Disabled),
        test_from_str_disabled_variant_casing_with_whitespace_is_valid: (" disaBLEd ", Flag::Disabled),
        test_from_str_disabled_variant_casing_with_lf_is_valid: ("\ndisaBLEd\n", Flag::Disabled),
        test_from_str_disabled_variant_casing_with_cr_is_valid: ("\rdisaBLEd\r", Flag::Disabled),
        test_from_str_disabled_variant_casing_with_crlf_is_valid: ("\r\ndisaBLEd\r\n", Flag::Disabled),
        test_from_str_admin_all_lowercase_is_valid:("admin", Flag::Admin),
        test_from_str_admin_all_lowercase_with_whitespace_is_valid: (" admin ", Flag::Admin),
        test_from_str_admin_all_lowercase_with_lf_is_valid: ("\nadmin\n", Flag::Admin),
        test_from_str_admin_all_lowercase_with_cr_is_valid: ("\radmin\r", Flag::Admin),
        test_from_str_admin_all_lowercase_with_crlf_is_valid: ("\r\nadmin\r\n", Flag::Admin),
        test_from_str_admin_all_uppercase_is_valid:("ADMIN", Flag::Admin),
        test_from_str_admin_all_uppercase_with_whitespace_is_valid: (" ADMIN ", Flag::Admin),
        test_from_str_admin_all_uppercase_with_lf_is_valid: ("\nADMIN\n", Flag::Admin),
        test_from_str_admin_all_uppercase_with_cr_is_valid: ("\rADMIN\r", Flag::Admin),
        test_from_str_admin_all_uppercase_with_crlf_is_valid: ("\r\nADMIN\r\n", Flag::Admin),
        test_from_str_admin_variant_casing_is_valid:("adMIn", Flag::Admin),
        test_from_str_admin_variant_casing_with_whitespace_is_valid: (" adMIn ", Flag::Admin),
        test_from_str_admin_variant_casing_with_lf_is_valid: ("\nadMIn\n", Flag::Admin),
        test_from_str_admin_variant_casing_with_cr_is_valid: ("\radMIn\r", Flag::Admin),
        test_from_str_admin_variant_casing_with_crlf_is_valid: ("\r\nadMIn\r\n", Flag::Admin),
        test_from_str_owner_all_lowercase_is_valid:("owner", Flag::Owner),
        test_from_str_owner_all_lowercase_with_whitespace_is_valid: (" owner ", Flag::Owner),
        test_from_str_owner_all_lowercase_with_lf_is_valid: ("\nowner\n", Flag::Owner),
        test_from_str_owner_all_lowercase_with_cr_is_valid: ("\rowner\r", Flag::Owner),
        test_from_str_owner_all_lowercase_with_crlf_is_valid: ("\r\nowner\r\n", Flag::Owner),
        test_from_str_owner_all_uppercase_is_valid:("OWNER", Flag::Owner),
        test_from_str_owner_all_uppercase_with_whitespace_is_valid: (" OWNER ", Flag::Owner),
        test_from_str_owner_all_uppercase_with_lf_is_valid: ("\nOWNER\n", Flag::Owner),
        test_from_str_owner_all_uppercase_with_cr_is_valid: ("\rOWNER\r", Flag::Owner),
        test_from_str_owner_all_uppercase_with_crlf_is_valid: ("\r\nOWNER\r\n", Flag::Owner),
        test_from_str_owner_variant_casing_is_valid:("owNER", Flag::Owner),
        test_from_str_owner_variant_casing_with_whitespace_is_valid: (" owNER ", Flag::Owner),
        test_from_str_owner_variant_casing_with_lf_is_valid: ("\nowNER\n", Flag::Owner),
        test_from_str_owner_variant_casing_with_cr_is_valid: ("\rowNER\r", Flag::Owner),
        test_from_str_owner_variant_casing_with_crlf_is_valid: ("\r\nowNER\r\n", Flag::Owner),
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
                let result = Flag::from_str(input).unwrap_err();
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
        
                let result = Flag::from_str(&enum_value).unwrap();
        
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
            ("deleted|", fixed_utc, Flag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_lowercase_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" deleted | ", fixed_utc, Flag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_lowercase_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\ndeleted\n|\n", fixed_utc, Flag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_lowercase_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rdeleted\r|\r", fixed_utc, Flag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_lowercase_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\ndeleted\r\n|\r\n", fixed_utc, Flag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_uppercase_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("DELETED|", fixed_utc, Flag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_uppercase_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" DELETED | ", fixed_utc, Flag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_uppercase_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rDELETED\n|\n", fixed_utc, Flag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_uppercase_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rDELETED\r|\r", fixed_utc, Flag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_uppercase_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\nDELETED\r\n|\r\n", fixed_utc, Flag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_variant_casing_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("DeLEted|", fixed_utc, Flag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_variant_casing_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" DeLEted | ", fixed_utc, Flag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_variant_casing_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rDeLEted\n|\n", fixed_utc, Flag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_variant_casing_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rDeLEted\r|\r", fixed_utc, Flag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_variant_casing_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\nDeLEted\r\n|\r\n", fixed_utc, Flag::Deleted { date: fixed_utc })
        },
        test_from_str_banned_all_lowercase_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("banned|", fixed_utc, Flag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_lowercase_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" banned | ", fixed_utc, Flag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_lowercase_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\nbanned\n|\n", fixed_utc, Flag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_lowercase_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rbanned\r|\r", fixed_utc, Flag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_lowercase_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\nbanned\r\n|\r\n", fixed_utc, Flag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_uppercase_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("BANNED|", fixed_utc, Flag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_uppercase_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" BANNED | ", fixed_utc, Flag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_uppercase_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rBANNED\n|\n", fixed_utc, Flag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_uppercase_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rBANNED\r|\r", fixed_utc, Flag::Banned { date: fixed_utc })
        },
        test_from_str_banned_all_uppercase_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\nBANNED\r\n|\r\n", fixed_utc, Flag::Banned { date: fixed_utc })
        },
        test_from_str_banned_variant_casing_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("baNNEd|", fixed_utc, Flag::Banned { date: fixed_utc })
        },
        test_from_str_banned_variant_casing_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" baNNEd | ", fixed_utc, Flag::Banned { date: fixed_utc })
        },
        test_from_str_banned_variant_casing_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rbaNNEd\n|\n", fixed_utc, Flag::Banned { date: fixed_utc })
        },
        test_from_str_banned_variant_casing_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rbaNNEd\r|\r", fixed_utc, Flag::Banned { date: fixed_utc })
        },
        test_from_str_banned_variant_casing_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\nbaNNEd\r\n|\r\n", fixed_utc, Flag::Banned { date: fixed_utc })
        },
    }
}