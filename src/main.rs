mod api;

use api::hello::{routes_hello};

use axum::{routing::Router};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let routes_all = Router::new()
    .merge(routes_hello());

    let listener = TcpListener::bind("127.0.0.1:8080")
    .await
    .unwrap();

    println!("{:<12} - {:?}", "Listening", listener.local_addr());

    axum::serve(listener, routes_all.into_make_service())
    .await
    .unwrap();

    Ok(())
}