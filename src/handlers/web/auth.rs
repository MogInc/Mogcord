mod login;
mod logout;

pub use login::*;
pub use logout::*;


fn is_logged_in(jar: &tower_cookies::Cookies) -> Result<(), crate::model::error::Client>
{
    let ctx_result = crate::middleware::auth::get_ctx(jar);

    if ctx_result.is_ok()
    {
        return Err(crate::model::error::Client::USER_ALREADY_LOGGED_IN);
    }

    Ok(())
}