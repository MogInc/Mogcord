use std::sync::Arc;

use axum::Router;

use crate::model::misc::AppState;

pub fn routes_relation(state: Arc<AppState>) -> Router
{
    Router::new()
}