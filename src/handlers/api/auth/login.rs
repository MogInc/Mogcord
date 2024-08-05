use std::net::SocketAddr;
use std::sync::Arc;
use axum::extract::ConnectInfo;
use axum::{extract::State, response::IntoResponse, Json};
use tower_cookies::Cookies;

use crate::handlers::logic;
use crate::handlers::logic::auth::LoginRequest;
use crate::model::AppState;

pub async fn login(
    State(state): State<Arc<AppState>>,
    jar: Cookies, 
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse
{
    logic::auth::login(&state, &jar, addr.to_string(), &payload).await
}