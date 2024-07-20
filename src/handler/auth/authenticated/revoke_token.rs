use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use tower_cookies::Cookies;

use crate::model::AppState;
use crate::middleware::{auth::{self, Ctx}, cookies::Manager};

//can see this as a logout
pub async fn revoke_token(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    jar: Cookies,
) -> impl IntoResponse
{
    let repo_refresh = &state.refresh_token;

    let device_id_cookie = jar.get_cookie(auth::CookieNames::DEVICE_ID.as_str())?;
    let ctx_user_id = &ctx.user_id_ref();

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
