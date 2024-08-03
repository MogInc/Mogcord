use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::handlers::logic;
use crate::model::AppState;

#[derive(Deserialize)]
pub struct LoginRequest
{
    mail: String,
    password: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    jar: Cookies, 
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse
{
    logic::auth::login(state, jar, &payload.mail, &payload.password).await
}