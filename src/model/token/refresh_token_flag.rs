use std::str::FromStr;
use serde::{de::{self, Visitor}, Deserialize, Serialize};
use std::fmt;


#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum RefreshTokenFlag 
{
    None,
    Revoked,
}

impl RefreshTokenFlag
{
    pub fn is_yeeted(&self) -> bool
    {
        match &self
        {
            Self::None => false,
            Self::Revoked => true,
        }
    }
}

impl fmt::Display for RefreshTokenFlag 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
        match self
        {
            _ => write!(f, "{self:?}")
        }
    }
}

impl<'de> Deserialize<'de> for RefreshTokenFlag
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> 
    {
        struct RefreshTokenFlagVisitor;

        impl<'de> Visitor<'de> for RefreshTokenFlagVisitor
        {
            type Value = RefreshTokenFlag;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result 
            {
                return formatter.write_str("data");
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where E: serde::de::Error, 
            {
                return RefreshTokenFlag::from_str(v)
                    .map_err(|_| de::Error::unknown_field(v, FIELDS));
            }
        }

        const FIELDS: &[&str] = &["none", "revoked"];
        return deserializer.deserialize_identifier(RefreshTokenFlagVisitor);
    }
}

impl FromStr for RefreshTokenFlag 
{
    type Err = RefreshTokenFlagParseError;

    fn from_str(input: &str) -> Result<RefreshTokenFlag, Self::Err> 
    {
        match input.to_lowercase().as_str() 
        {
            "none" => Ok(RefreshTokenFlag::None),
            "revoked" => Ok(RefreshTokenFlag::Revoked),
            _ => Err(RefreshTokenFlagParseError::InvalidFormat),
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum RefreshTokenFlagParseError 
{
    InvalidFormat,
}

impl fmt::Display for RefreshTokenFlagParseError 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        match *self 
        {
            RefreshTokenFlagParseError::InvalidFormat => write!(f, "Invalid format"),
        }
    }
}

impl std::error::Error for RefreshTokenFlagParseError {}