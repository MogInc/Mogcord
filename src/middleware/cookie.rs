use strum_macros::Display;

#[derive(Display)]
#[allow(non_camel_case_types)]
pub enum CookieNames
{
    AUTH_TOKEN,
    SESSION_TOKEN,
    DEVICE_ID,
}

impl CookieNames 
{
    fn as_str(&self) -> &'static str 
    {
        match self 
        {
            CookieNames::AUTH_TOKEN => "AUTH_TOKEN",
            CookieNames::SESSION_TOKEN => "SESSION_TOKEN",
            CookieNames::DEVICE_ID => "DEVICE_ID",
        }
    }
}

impl From<CookieNames> for &'static str
{
    fn from(value: CookieNames) -> Self 
    {
        &value.as_str()
    }
}