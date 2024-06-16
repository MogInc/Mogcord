mod api;
mod db;

use std::sync::Arc;

use db::mongoldb::mongoldb::MongolDB;
use api::user::{routes_user};

use axum::{http::StatusCode, response::IntoResponse, routing::Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mongodb_address = "mongodb://localhost:27017";
    let address = "127.0.0.1:8080";

    let db = MongolDB::init(mongodb_address).await?;
    let client = db.client;
    let client_arc: Arc<_> = Arc::new(client);


    let api_routes = Router::new()
    .merge(routes_user(client_arc));


    let app = Router::new()
    .nest("/api", api_routes)
    .fallback(page_not_found);


    let listener = TcpListener::bind(address)
    .await
    .unwrap();


    println!("{:<12} - {:?}", "Listening", listener.local_addr());


    axum::serve(listener, app.into_make_service())
    .await
    .unwrap();

    Ok(())
}

async fn page_not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 Page Not Found")
}