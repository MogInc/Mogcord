use std::sync::Arc;

use axum::{response::Html, routing::get, Router};

use crate::model::AppState;

pub async fn handler() -> Html<String> 
{
    Html("<h1>Hello there test</h1>".to_string())
}

pub fn routes(state: Arc<AppState>) -> Router
{
    let routes_without_middleware =  Router::new()
        //hello
        .route("/hello", get(handler))
        .with_state(state);


    Router::new()
        .merge(routes_without_middleware)
}
