mod cookies;
mod auth_cookie_names;

pub use cookies::*;
pub use auth_cookie_names::*;

pub const JWT_COOKIE_TTL_MINS: i64 = 60 * 24 * 31;