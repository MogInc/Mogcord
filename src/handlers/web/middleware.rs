use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::middleware::auth::Ctx;
use crate::model::error as m_error;

use super::model::HtmxError;

pub async fn mw_require_htmx_authentication(
    ctx: m_error::Result<'_, Ctx>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, HtmxError>
{
    println!("AUTH HTMX (REG): ");

    if ctx.is_err()
    {
        return Err(HtmxError::new(m_error::Client::PERMISSION_NO_AUTH));
    }

    Ok(next.run(req).await)
}
