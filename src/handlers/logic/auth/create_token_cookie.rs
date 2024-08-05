use tower_cookies::Cookies;

use crate::middleware::cookies::Manager;
use crate::model::error;
use crate::model::refresh_token::RefreshToken;
use crate::middleware::auth::{self, CreateAccesTokenRequest};


pub fn create_token_cookie<'err>(
    jar: &Cookies,
    refresh_token: RefreshToken,
    acces_token_request: &CreateAccesTokenRequest,
) -> error::Result<'err, ()>
{
    match auth::create_acces_token(acces_token_request)
    {
        Ok(acces_token) => 
        {
            let cookie_names_acces_token = auth::CookieNames::AUTH_ACCES;
            let cookie_names_refresh_token = auth::CookieNames::AUTH_REFRESH;
            let cookie_names_device_id = auth::CookieNames::DEVICE_ID;
        
            jar.create_cookie(
                cookie_names_acces_token.to_string(), 
                acces_token, 
                cookie_names_acces_token.ttl_in_mins(), 
            );
            
            jar.create_cookie(
                cookie_names_refresh_token.to_string(),
                refresh_token.value,
                cookie_names_refresh_token.ttl_in_mins(),
            );
            
            jar.create_cookie(
                cookie_names_device_id.to_string(),
                refresh_token.device_id,
                cookie_names_device_id.ttl_in_mins(),
            );

            Ok(())
        },
        Err(err) => Err(err),
    }
}