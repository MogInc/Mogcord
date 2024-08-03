use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use axum_htmx::HxRedirect;
use tower_cookies::Cookies;

use crate::{handlers::logic, middleware::auth::Ctx, model::{error, AppState}};

pub async fn logout(
    State(state): State<Arc<AppState>>,
    jar: Cookies,
    ctx: Ctx,
) -> Result<impl IntoResponse, error::Client>
{
    logic::auth::authenticated::revoke_token(&state, &ctx, &jar).await
        .map_err(|err| err.client)?;

    Ok((HxRedirect("/".parse().unwrap()), "").into_response())
}