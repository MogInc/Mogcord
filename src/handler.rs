use std::time::Duration;
use std::{collections::HashMap, sync::Arc};
use axum::error_handling::HandleErrorLayer;
use axum::{RequestPartsExt, Router};
use axum::routing::{delete, get, patch, post};
use axum::response::{IntoResponse, Response};
use axum::middleware;
use axum::http::{request::Parts, StatusCode};
use axum::extract::{FromRequestParts, Path};
use axum::async_trait;
use tower::{BoxError, ServiceBuilder};
use tower::{buffer::BufferLayer, limit::RateLimitLayer};

use crate::{middleware::auth::mw_require_admin_authentication, model::AppState};
use crate::middleware::auth::{mw_ctx_resolver, mw_require_authentication};

pub mod user;
pub mod chat;
pub mod server;
pub mod message;
pub mod auth;
pub mod relation;


pub fn routes(state: Arc<AppState>) -> Router
{
    let routes_with_admin_middleware = Router::new()
        //users
        .route("/admin/users/:user_id", get(user::admin::get_user))
        .route("/admin/users", get(user::admin::get_users))
        .with_state(state.clone())
        .route_layer(middleware::from_fn(mw_require_admin_authentication))
        .route_layer(middleware::from_fn(mw_ctx_resolver));

    let routes_with_regular_middleware =  Router::new()
        //auth
        .route("/auth/revoke", delete(auth::authenticated::revoke_token))
        .route("/auth/revoke/all", delete(auth::authenticated::revoke_all_tokens))
        //chat
        .route("/chat", post(chat::authenticated::create_chat))
        .route("/chat/:chat_id", get(chat::authenticated::get_chat))
        .route("/chat/:chat_id/users", post(chat::authenticated::add_users_to_chat))
        //messages
        .route("/channels/:channel_id/messages", get(message::authenticated::get_messages))
        .route("/channels/:channel_id/messages", post(message::authenticated::create_message))
        .route("/channels/:channel_id/messages/:message_id", patch(message::authenticated::update_message))
        //relations
        .route("/friends", post(relation::authenticated::add_friend))
        .route("/friends/confirm", post(relation::authenticated::confirm_friend))
        .route("/friends", delete(relation::authenticated::remove_friend))
        .route("/blocked", post(relation::authenticated::add_blocked))
        .route("/blocked", delete(relation::authenticated::remove_blocked))
        //servers
        .route("/servers", post(server::authenticated::create_server))
        .route("/servers/:server_id", get(server::authenticated::get_server))
        .route("/servers/:server_id/join", post(server::authenticated::join_server))
        //users
        .route("/users/current", get(user::authenticated::get_ctx_user_auth))
        .route_layer(middleware::from_fn(mw_require_authentication))
        .route_layer(middleware::from_fn(mw_ctx_resolver))
        .with_state(state.clone());


    let routes_without_middleware =  Router::new()
        //auth
        .route("/auth/login", post(auth::login).layer(
            ServiceBuilder::new()
            .layer(HandleErrorLayer::new(handle_too_many_requests))
            .layer(BufferLayer::new(1024))
            .layer(RateLimitLayer::new(Limit::Login.attempts(), Limit::Login.duration()))
        ))
        .route("/auth/refresh", post(auth::refresh_token))
        //users
        .route("/users", post(user::create_user))
        .with_state(state);


    Router::new()
        .merge(routes_with_admin_middleware)
        .merge(routes_with_regular_middleware)
        .merge(routes_without_middleware)
}

enum Limit
{
    Login,
}

impl Limit
{
    const MIN: u64 = 60;

    fn attempts(&self) -> u64
    {
        match self
        {
            Limit::Login => 5,
        }
    }

    fn duration(&self) -> Duration
    {
        match self
        {
            Limit::Login => Duration::from_secs(5 * Self::MIN),
        }
    }
}

async fn handle_too_many_requests(err: BoxError) -> (StatusCode, String) 
{
    (
        StatusCode::TOO_MANY_REQUESTS,
        format!("To many requests: {err}")
    )
}

#[derive(Debug)]
pub enum Version 
{
    V1,
}

#[async_trait]
impl<S> FromRequestParts<S> for Version
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> 
    {
        let params: Path<HashMap<String, String>> =
            parts.extract().await.map_err(IntoResponse::into_response)?;

        let version = params
            .get("version")
            .ok_or_else(|| (StatusCode::NOT_FOUND, "version param missing").into_response())?;

        match version.to_lowercase().as_str() 
        {
            "v1" => Ok(Version::V1),
            _ => Err((StatusCode::NOT_FOUND, "unknown version").into_response()),
        }
    }
}