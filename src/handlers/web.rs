mod auth;

use std::sync::Arc;
use askama::Template;
use axum::{routing::get, Router};
use tower_http::services::ServeFile;

use crate::model::AppState;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

pub async fn index() -> Index
{
    Index
    {
        
    }
}

pub fn routes(state: Arc<AppState>) -> Router
{
    let routes_without_middleware =  Router::new()
        //auth
        .route("/login", get(auth::login))
        //hello
        .route("/", get(index))
        .nest_service("/main.css", ServeFile::new("templates/main.css"))
        .with_state(state);


    Router::new()
        .merge(routes_without_middleware)
}
