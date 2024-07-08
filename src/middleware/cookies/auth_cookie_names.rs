use serde::Serialize;
use strum_macros::Display;

#[derive(Display, Serialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum AuthCookieNames
{
    AUTH_ACCES,
    AUTH_REFRESH,
    DEVICE_ID,
}

impl AuthCookieNames
{
    pub fn as_str(&self) -> &'static str 
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
            AuthCookieNames::AUTH_ACCES => 60 * 24 * 31,
            AuthCookieNames::AUTH_REFRESH => 60 * 24 * 365,
            AuthCookieNames::DEVICE_ID => 60 * 24 * 365 * 5,
        }
    }
}

impl From<AuthCookieNames> for &'static str
{
    fn from(value: AuthCookieNames) -> Self 
    {
        &value.as_str()
    }
}