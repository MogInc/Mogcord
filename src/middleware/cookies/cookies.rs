use tower_cookies::{cookie::{time::{Duration, OffsetDateTime}, SameSite}, Cookie, Cookies};

pub trait Cookie2
{
    fn create_cookie(&self, name: &str, value: String, ttl_in_mins: i64);
    fn get_cookie(&self, name: &str) -> Option<String>;
    fn remove_cookie(&self, name: &str);
}


impl Cookie2 for Cookies
{
    fn create_cookie(&self, name: &str, value: String, ttl_in_mins: i64) 
    {
        let cookie = Cookie::build((name.to_string(), value))
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Lax)
            .expires(OffsetDateTime::now_utc() + Duration::minutes(ttl_in_mins))
            .build();

        self.add(cookie);
    }

    fn get_cookie(&self, name: &str) -> Option<String> {
        return self
            .get(name)
            .map(|c| c.value().to_string());
    }

    fn remove_cookie(&self, name: &str) {
        let cookie = Cookie::build(name.to_string()).path("/").build();

        self.remove(cookie);
    }
}