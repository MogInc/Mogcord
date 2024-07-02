use strum_macros::Display;

#[derive(Display)]
#[allow(non_camel_case_types)]
pub enum CookieNames
{
    AUTH_TOKEN,
    SESSION_TOKEN,
    DEVICE_ID,
}