use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum_htmx::HxRedirect;
use tower_cookies::Cookies;

use crate::handlers::logic;
use crate::handlers::web::model::HtmxError;
use crate::middleware::auth::Ctx;
use crate::model::AppState;

pub async fn logout(
    State(state): State<Arc<AppState>>,
    jar: Cookies,
    ctx: Ctx,
) -> Result<impl IntoResponse, HtmxError>
{
    logic::auth::authenticated::revoke_token(&state, &ctx, &jar)
        .await
        .map_err(|err| HtmxError::new(err.client))?;

    Ok((HxRedirect("/".parse().unwrap()), "").into_response())
}
