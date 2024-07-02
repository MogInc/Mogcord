use axum::{body::Body, http::Request, middleware::Next, response::Response};

use crate::model::misc::ServerError;


pub async fn mw_require_auth(req: Request<Body>, next: Next) -> Result<Response, ServerError>
{
    println!("AUTH MIDDLEWARE");

    return Ok(next.run(req).await);
}