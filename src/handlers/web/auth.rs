pub mod authenticate;
mod login;

pub use login::*;


fn is_logged_in(jar: &tower_cookies::Cookies) -> Result<(), super::HtmxError>
{
    let ctx_result = crate::middleware::auth::get_ctx(jar);

    if ctx_result.is_ok()
    {
        return Err(super::HtmxError::new(crate::model::error::Client::USER_ALREADY_LOGGED_IN));
    }

    Ok(())
}