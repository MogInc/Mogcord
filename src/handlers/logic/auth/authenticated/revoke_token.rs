use std::sync::Arc;
use tower_cookies::Cookies;

use crate::model::{error, AppState};
use crate::middleware::{auth::{self, Ctx}, cookies::Manager};
use crate::server_error;

//can see this as a logout
pub async fn revoke_token<'err>(
    state: &Arc<AppState>,
    ctx: &Ctx,
    jar: &Cookies,
) -> Result<(), error::Server<'err>>
{
    let repo_refresh = &state.refresh_tokens;

    let device_id_cookie = jar.get_cookie(auth::CookieNames::DEVICE_ID.as_str())
        .map_err(|err| server_error!(err, error::Kind::NoAuth, error::OnType::Cookie))?;
    
    let ctx_user_id = ctx.user_id_ref();

    match repo_refresh.revoke_token(ctx_user_id, &device_id_cookie).await
    {
        Ok(()) => 
        {
            jar.remove_cookie(auth::CookieNames::AUTH_ACCES.to_string());
            jar.remove_cookie(auth::CookieNames::AUTH_REFRESH.to_string());

            Ok(())
        },
        Err(err) => Err(err),
    }
}
