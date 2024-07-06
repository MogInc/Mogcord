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

    pub fn get_cookie(jar: &Cookies, name: &str) -> Option<String>
    {
        return jar
            .get(name)
            .map(|c| c.value().to_string());
    }

    pub fn remove_cookie(jar: &Cookies, name: &'static str)
    {
        let cookie = Cookie::build(name).path("/").build();

        jar.remove(cookie);

        let test = Cookie::from(name);
        println!("{:?}", test);
    }
}