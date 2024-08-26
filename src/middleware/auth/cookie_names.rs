use serde::Serialize;
use std::fmt;

#[derive(Serialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum CookieNames
{
    AUTH_ACCES,
    AUTH_REFRESH,
    DEVICE_ID,
}

impl fmt::Display for CookieNames
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.as_str())
    }
}

impl CookieNames
{
    #[must_use]
    pub fn as_str(&self) -> &str
    {
        match self
        {
            CookieNames::AUTH_ACCES => "ACCES_TOKEN",
            CookieNames::AUTH_REFRESH => "SESSION_TOKEN",
            CookieNames::DEVICE_ID => "DEVICE_ID",
        }
    }

    #[must_use]
    pub fn ttl_in_mins(&self) -> i64
    {
        match self
        {
            CookieNames::AUTH_ACCES | CookieNames::AUTH_REFRESH => 60 * 24 * 365,
            CookieNames::DEVICE_ID => 60 * 24 * 365 * 5,
        }
    }
}
