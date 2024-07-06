use tower_cookies::{cookie::{time::{Duration, OffsetDateTime}, SameSite}, Cookie, Cookies};

pub struct CookieManager;

impl CookieManager
{

    pub fn create_cookie<'a>(name: &'a str, value: String, ttl_in_mins: i64) -> Cookie<'a>
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

    pub fn get_cookie(cookies: &Cookies, name: &str) -> Option<String>
    {
        return cookies
            .get(name)
            .map(|c| c.value().to_string());
    }

    pub fn remove_cookie(cookies: &Cookies, name: &'static str)
    {
        cookies.remove(Cookie::from(name))
    }
}