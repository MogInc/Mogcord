use serde::Serialize;
use strum_macros::Display;

#[derive(Display, Serialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum AuthCookieNames
{
    AUTH_TOKEN,
    AUTH_REFRESH,
    DEVICE_ID,
}

impl AuthCookieNames 
{
    fn as_str(&self) -> &'static str 
    {
        match self 
        {
            AuthCookieNames::AUTH_TOKEN => "ACCES_TOKEN",
            AuthCookieNames::AUTH_REFRESH => "SESSION_TOKEN",
            AuthCookieNames::DEVICE_ID => "DEVICE_ID",
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