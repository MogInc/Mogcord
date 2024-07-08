pub mod jwt;


mod mw_auth;
mod ctx;

pub use mw_auth::*;
pub use ctx::*;

pub const ACCES_TOKEN_TTL_MIN: i64 = 10;
pub const REFRESH_TOKEN_TTL_MIN: i64 = 60 * 24 * 365;
pub const DEVICE_ID_TTL_MIN: i64 = 60 * 24 * 365 * 5;