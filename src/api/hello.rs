use axum::{response::{Html, IntoResponse}, routing::get, Router};

pub fn routes_hello() -> Router
{
    return Router::new().route(
        "/hello",
        get(get_hello),
    );
}

async fn get_hello() -> impl IntoResponse
{
    Html("Hello world")
}