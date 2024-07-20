use std::sync::Arc;

use axum::{middleware, routing::{delete, get, post}, Router};

use crate::{middleware::auth::{mw_ctx_resolver, mw_require_regular_auth}, model::AppState};

pub mod user;
pub mod chat;
pub mod server;
pub mod message;
pub mod auth;
pub mod relation;

pub fn routes(state: Arc<AppState>) -> Router
{
    let routes_without_middleware =  Router::new()
        .route("/auth/login", post(auth::login_for_everyone))
        .route("/auth/refresh", post(auth::refresh_token_for_everyone))
        .with_state(state.clone());

    let routes_with_regular_middleware =  Router::new()
        .route("/auth/revoke", delete(auth::revoke_token_for_authorized))
        .route("/auth/revoke/all", delete(auth::revoke_all_tokens_for_authorized))
        .route("/chat", post(chat::create_chat_for_authenticated))
        .route("/chat/:chat_id", get(chat::get_chat_for_authenticated))
        .route("/chat/:chat_id/users", post(chat::add_users_to_chat_for_authenticated))
        .layer(middleware::from_fn(mw_require_regular_auth))
        .layer(middleware::from_fn(mw_ctx_resolver))
        .with_state(state);


    Router::new()
        .merge(routes_with_regular_middleware)
        .merge(routes_without_middleware)
}