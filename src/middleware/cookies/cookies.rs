use tower_cookies::{cookie::{time::{Duration, OffsetDateTime}, SameSite}, Cookie};

pub fn create_cookie<'a>(name: String, value: String, ttl_in_mins: i64) -> Cookie<'a>
{
    let cookie = Cookie::build((name, value))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .expires(OffsetDateTime::now_utc() + Duration::minutes(ttl_in_mins))
        .build();

    return cookie;
}