use axum::response::{Html, IntoResponse};

pub async fn get_hello() -> impl IntoResponse
{
    Html("Hello world")
}