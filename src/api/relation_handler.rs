use std::sync::Arc;

use axum::{extract::{self, State}, middleware, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;

use crate::{middleware::auth::{self, Ctx}, model::misc::{AppState, ServerError}};

pub fn routes_relation(state: Arc<AppState>) -> Router
{
    Router::new()
        .route("/friends", todo!())
        .route("/friends", post(add_friend_for_authenticated))
        .route("/blocked", todo!())
        .route("/blocked", todo!())
        .with_state(state)
        .route_layer(middleware::from_fn(auth::mw_require_regular_auth))
        .route_layer(middleware::from_fn(auth::mw_ctx_resolver))
}

#[derive(Deserialize)]
struct RelationRequest
{
    user_id: String,
}


async fn add_friend_for_authenticated(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(payload): Json<RelationRequest>,
) -> impl IntoResponse
{
    
    todo!()
}