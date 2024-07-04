mod cookies;
mod auth_cookie_names;

pub use cookies::*;
pub use auth_cookie_names::*;

pub const COOKIE_ACCES_TOKEN_TTL_MIN: i64 = 60 * 24 * 31;
pub const COOKIE_REFRESH_TOKEN_TTL_MIN: i64 = 60 * 24 * 365;
pub const COOKIE_DEVICE_ID_TTL_MIN: i64 = 60 * 24 * 365 * 5;