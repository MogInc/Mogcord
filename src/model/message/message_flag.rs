use std::{fmt, str::FromStr};
use chrono::{DateTime, Utc};
use serde::{de::{self, Visitor}, Deserialize, Serialize};



#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum MessageFlag
{
    None,
    Edited { date: DateTime<Utc> },
    Deleted { date: DateTime<Utc> },
}

impl MessageFlag
{
    pub fn is_allowed_to_be_editted(&self) -> bool
    {
        match &self
        {
            Self::None | Self::Edited { .. } => true,
            _ => false,
        }
    }
}

impl fmt::Display for MessageFlag 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
        match self
        {
            Self::None => write!(f, "none"),
            Self::Edited { date } => write!(f, "edited|{}", date),
            Self::Deleted { date } => write!(f, "deleted|{}", date),
        }
    }
}

impl<'de> Deserialize<'de> for MessageFlag
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> 
    {
        struct MessageFlagVisitor;

        impl<'de> Visitor<'de> for MessageFlagVisitor
        {
            type Value = MessageFlag;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result 
            {
                return formatter.write_str("data");
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where E: serde::de::Error, 
            {
                return MessageFlag::from_str(v)
                    .map_err(|_| de::Error::unknown_field(v, FIELDS));
            }
        }

        const FIELDS: &[&str] = &["none", "edited", "deleted"];
        
        deserializer.deserialize_identifier(MessageFlagVisitor)
    }
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

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests
{
    use std::str::FromStr;
    use chrono::Utc;

    use crate::model::message::{MessageFlag, MessageFlagParseError};

    macro_rules! from_str_base_tests_valid
    {
        ($($name:ident: $value:expr,)*) => 
        {
        $(
            #[test]
            fn $name() 
            {
                let (input, expected) = $value;
                let result = MessageFlag::from_str(input).unwrap();
                assert_eq!(result, expected);
            }
        )*
        }
    }
    
    from_str_base_tests_valid! 
    {
        test_from_str_none_all_lowercase_is_valid: ("none", MessageFlag::None),
        test_from_str_none_all_lowercase_with_whitespace_is_valid: (" none ", MessageFlag::None),
        test_from_str_none_all_lowercase_with_lf_is_valid: ("\nnone\n", MessageFlag::None),
        test_from_str_none_all_lowercase_with_cr_is_valid: ("\rnone\r", MessageFlag::None),
        test_from_str_none_all_lowercase_with_crlf_is_valid: ("\r\nnone\r\n", MessageFlag::None),
        test_from_str_none_all_uppercase_is_valid: ("NONE", MessageFlag::None),
        test_from_str_none_all_uppercase_with_whitespace_is_valid: (" NONE ", MessageFlag::None),
        test_from_str_none_all_uppercase_with_lf_is_valid: ("\nNONE\n", MessageFlag::None),
        test_from_str_none_all_uppercase_with_cr_is_valid: ("\rNONE\r", MessageFlag::None),
        test_from_str_none_all_uppercase_with_crlf_is_valid: ("\r\nNONE\r\n", MessageFlag::None),
        test_from_str_none_variant_casing_is_valid: ("nONe", MessageFlag::None),
        test_from_str_none_variant_casing_with_whitespace_is_valid: (" nONe ", MessageFlag::None),
        test_from_str_none_variant_casing_with_lf_is_valid: ("\nnONe\n", MessageFlag::None),
        test_from_str_none_variant_casing_with_cr_is_valid: ("\rnONe\r", MessageFlag::None),
        test_from_str_none_variant_casing_with_crlf_is_valid: ("\r\nnONe\r\n", MessageFlag::None),
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
                let result = MessageFlag::from_str(input).unwrap_err();
                assert_eq!(result, expected);
            }
        )*
        }
    }

    from_str_base_tests_invalid! 
    {
        test_from_str_is_empty:("", MessageFlagParseError::InvalidFormat),
        test_from_str_is_invalid:("AAAaaa", MessageFlagParseError::InvalidFormat),
        test_from_str_edited_invalid_format_is_invalid:("edited", MessageFlagParseError::InvalidFormat),
        test_from_str_edited_invalid_separator_is_invalid:("edited+utc_time", MessageFlagParseError::InvalidFormat),
        test_from_str_edited_invalid_date_is_invalid:("edited|utc_time", MessageFlagParseError::InvalidDate),
        test_from_str_deleted_invalid_format_is_invalid:("deleted", MessageFlagParseError::InvalidFormat),
        test_from_str_deleted_invalid_separator_is_invalid:("deleted+utc_time", MessageFlagParseError::InvalidFormat),
        test_from_str_deleted_invalid_date_is_invalid:("deleted|utc_time", MessageFlagParseError::InvalidDate),
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
        
                let result = MessageFlag::from_str(&enum_value).unwrap();
        
                assert_eq!(result, expected);
            }
        )*
        }
    }

    from_str_date_tests_valid! 
    {
        test_from_str_edited_all_lowercase_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("edited|", fixed_utc, MessageFlag::Edited { date: fixed_utc })
        },
        test_from_str_edited_all_lowercase_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" edited | ", fixed_utc, MessageFlag::Edited { date: fixed_utc })
        },
        test_from_str_edited_all_lowercase_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\nedited\n|\n", fixed_utc, MessageFlag::Edited { date: fixed_utc })
        },
        test_from_str_edited_all_lowercase_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\redited\r|\r", fixed_utc, MessageFlag::Edited { date: fixed_utc })
        },
        test_from_str_edited_all_lowercase_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\nedited\r\n|\r\n", fixed_utc, MessageFlag::Edited { date: fixed_utc })
        },
        test_from_str_edited_all_uppercase_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("EDITED|", fixed_utc, MessageFlag::Edited { date: fixed_utc })
        },
        test_from_str_edited_all_uppercase_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" EDITED | ", fixed_utc, MessageFlag::Edited { date: fixed_utc })
        },
        test_from_str_edited_all_uppercase_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rEDITED\n|\n", fixed_utc, MessageFlag::Edited { date: fixed_utc })
        },
        test_from_str_edited_all_uppercase_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rEDITED\r|\r", fixed_utc, MessageFlag::Edited { date: fixed_utc })
        },
        test_from_str_edited_all_uppercase_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\nEDITED\r\n|\r\n", fixed_utc, MessageFlag::Edited { date: fixed_utc })
        },
        test_from_str_edited_variant_casing_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("ediTED|", fixed_utc, MessageFlag::Edited { date: fixed_utc })
        },
        test_from_str_edited_variant_casing_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" ediTED | ", fixed_utc, MessageFlag::Edited { date: fixed_utc })
        },
        test_from_str_edited_variant_casing_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rediTED\n|\n", fixed_utc, MessageFlag::Edited { date: fixed_utc })
        },
        test_from_str_edited_variant_casing_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rediTED\r|\r", fixed_utc, MessageFlag::Edited { date: fixed_utc })
        },
        test_from_str_edited_variant_casing_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\nediTED\r\n|\r\n", fixed_utc, MessageFlag::Edited { date: fixed_utc })
        },
        test_from_str_deleted_all_lowercase_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("deleted|", fixed_utc, MessageFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_lowercase_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" deleted | ", fixed_utc, MessageFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_lowercase_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\ndeleted\n|\n", fixed_utc, MessageFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_lowercase_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rdeleted\r|\r", fixed_utc, MessageFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_lowercase_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\ndeleted\r\n|\r\n", fixed_utc, MessageFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_uppercase_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("DELETED|", fixed_utc, MessageFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_uppercase_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" DELETED | ", fixed_utc, MessageFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_uppercase_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rDELETED\n|\n", fixed_utc, MessageFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_uppercase_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rDELETED\r|\r", fixed_utc, MessageFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_all_uppercase_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\nDELETED\r\n|\r\n", fixed_utc, MessageFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_variant_casing_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("deLEted|", fixed_utc, MessageFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_variant_casing_with_whitespace_is_valid: 
        {
            let fixed_utc = Utc::now();
            (" deLEted | ", fixed_utc, MessageFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_variant_casing_with_lf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rdeLEted\n|\n", fixed_utc, MessageFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_variant_casing_with_cr_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\rdeLEted\r|\r", fixed_utc, MessageFlag::Deleted { date: fixed_utc })
        },
        test_from_str_deleted_variant_casing_with_crlf_is_valid: 
        {
            let fixed_utc = Utc::now();
            ("\r\ndeLEted\r\n|\r\n", fixed_utc, MessageFlag::Deleted { date: fixed_utc })
        },
    }
}