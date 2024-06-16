mod api;

use api::user::{routes_user};

use axum::routing::Router;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() -> std::io::Result<()> {

    let api_routes = Router::new()
    .merge(routes_user());

    let app = Router::new()
    .nest("/api", api_routes);

    let listener = TcpListener::bind("127.0.0.1:8080")
    .await
    .unwrap();

    println!("{:<12} - {:?}", "Listening", listener.local_addr());

    axum::serve(listener, app.into_make_service())
    .await
    .unwrap();

    Ok(())
}