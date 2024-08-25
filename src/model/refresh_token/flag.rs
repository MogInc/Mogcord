use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Flag
{
    None,
    //can add utc date
    Revoked,
}

impl Flag
{
    #[must_use]
    pub fn is_yeeted(&self) -> bool
    {
        match &self
        {
            Self::None => false,
            Self::Revoked => true,
        }
    }
}

impl fmt::Display for Flag
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result
    {
        match self
        {
            Self::None => write!(f, "none"),
            Self::Revoked => write!(f, "revoked"),
        }
    }
}

impl<'de> Deserialize<'de> for Flag
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RefreshTokenFlagVisitor;

        impl<'de> Visitor<'de> for RefreshTokenFlagVisitor
        {
            type Value = Flag;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result
            {
                formatter.write_str("data")
            }

            fn visit_str<E>(
                self,
                v: &str,
            ) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Flag::from_str(v)
                    .map_err(|_| de::Error::unknown_field(v, FIELDS))
            }
        }

        const FIELDS: &[&str] = &["none", "revoked"];

        deserializer.deserialize_identifier(RefreshTokenFlagVisitor)
    }
}

impl FromStr for Flag
{
    type Err = RefreshTokenFlagParseError;

    fn from_str(input: &str) -> Result<Flag, Self::Err>
    {
        match input.to_lowercase().as_str()
        {
            "none" => Ok(Flag::None),
            "revoked" => Ok(Flag::Revoked),
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
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result
    {
        match *self
        {
            RefreshTokenFlagParseError::InvalidFormat =>
            {
                write!(f, "Invalid format")
            },
        }
    }
}

impl std::error::Error for RefreshTokenFlagParseError {}
