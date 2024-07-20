use std::sync::Arc;
use axum::{middleware, routing::{delete, get, patch, post}, Router};

use crate::{middleware::auth::mw_require_admin_auth, model::AppState};
use crate::middleware::auth::{mw_ctx_resolver, mw_require_regular_auth};

pub mod user;
pub mod chat;
pub mod server;
pub mod message;
pub mod auth;
pub mod relation;

pub fn routes(state: Arc<AppState>) -> Router
{
    let routes_with_admin_middleware = Router::new()
        //user
        .route("/admin/user/:user_id", get(user::get_user_admin))
        .route("/admin/users", get(user::get_users_admin))
        .with_state(state.clone())
        .route_layer(middleware::from_fn(mw_require_admin_auth))
        .route_layer(middleware::from_fn(mw_ctx_resolver));

    let routes_with_regular_middleware =  Router::new()
        //auth
        .route("/auth/revoke", delete(auth::revoke_token_auth))
        .route("/auth/revoke/all", delete(auth::revoke_all_tokens_auth))
        //chat
        .route("/chat", post(chat::create_chat_auth))
        .route("/chat/:chat_id", get(chat::get_chat_auth))
        .route("/chat/:chat_id/users", post(chat::add_users_to_chat_auth))
        //message
        .route("/chat/:chat_info_id/messages", get(message::get_messages_auth))
        .route("/chat/:chat_info_id/message", post(message::create_message_auth))
        .route("/chat/:chat_info_id/message/:message_id", patch(message::update_message_auth))
        //relation
        .route("/friends", post(relation::add_friend_auth))
        .route("/friends/confirm", post(relation::confirm_friend_auth))
        .route("/friends", delete(relation::remove_friend_auth))
        .route("/blocked", post(relation::add_blocked_auth))
        .route("/blocked", delete(relation::remove_blocked_auth))
        //server
        .route("/server", post(server::create_server_auth))
        .route("/server/:server_id", get(server::get_server_auth))
        .route("/server/:server_id/join", post(server::join_server_auth))
        //user
        .route("/user", get(user::get_ctx_user_auth))
        .route_layer(middleware::from_fn(mw_require_regular_auth))
        .route_layer(middleware::from_fn(mw_ctx_resolver))
        .with_state(state.clone());

    let routes_without_middleware =  Router::new()
        //auth
        .route("/auth/login", post(auth::login))
        .route("/auth/refresh", post(auth::refresh_token))
        //user
        .route("/user", post(user::create_user))
        .with_state(state);

    Router::new()
        .merge(routes_with_admin_middleware)
        .merge(routes_with_regular_middleware)
        .merge(routes_without_middleware)
}