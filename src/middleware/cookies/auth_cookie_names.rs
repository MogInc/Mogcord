use std::fmt;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum AuthCookieNames
{
    AUTH_ACCES,
    AUTH_REFRESH,
    DEVICE_ID,
}

impl fmt::Display for AuthCookieNames 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
        write!(f, "{}", self.as_str())
    }
}

impl AuthCookieNames
{
    pub fn as_str(&self) -> &str 
    {
        match self 
        {
            AuthCookieNames::AUTH_ACCES => "ACCES_TOKEN",
            AuthCookieNames::AUTH_REFRESH => "SESSION_TOKEN",
            AuthCookieNames::DEVICE_ID => "DEVICE_ID",
        }
    }

    pub fn ttl_in_mins(&self) -> i64
    {
        match self 
        {
            AuthCookieNames::AUTH_ACCES => 60 * 24 * 365,
            AuthCookieNames::AUTH_REFRESH => 60 * 24 * 365,
            AuthCookieNames::DEVICE_ID => 60 * 24 * 365 * 5,
        }
    }
}