use tower_cookies::{cookie::{time::{Duration, OffsetDateTime}, SameSite}, Cookie, Cookies};

use crate::model::error;

pub trait Cookie2
{
    fn create_cookie(&self, name: String, value: String, ttl_in_mins: i64);
    fn get_cookie(&self, name: &str) -> Result<String, error::Server>;
    fn remove_cookie(&self, name: String);
}


impl Cookie2 for Cookies
{
    fn create_cookie(&self, name: String, value: String, ttl_in_mins: i64) 
    {
        let cookie = Cookie::build((name, value))
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Lax)
            .expires(OffsetDateTime::now_utc() + Duration::minutes(ttl_in_mins))
            .build();

        self.add(cookie);
    }

    fn get_cookie(&self, name: &str) -> Result<String, error::Server> 
    {
        self
            .get(name)
            .map(|c| c.value().to_string())
            .ok_or(error::Server::CookieNotFound(name.to_string()))
    }

    fn remove_cookie(&self, name: String) 
    {
        let cookie = Cookie::build(name).path("/").build();

        self.remove(cookie);
    }
}