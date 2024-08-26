use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use mogcord::handlers;
use mogcord::model::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    dotenv().ok();

    let mongoldb_connection_string =
        env::var("MONGOLDB_CONNECTION").unwrap_or("mongodb://localhost:27017".to_string());

    let api_socket = env::var("API_SOCKET").unwrap_or("127.0.0.1:3000".to_string());

    let log_path = env::var("LOG_PATH").unwrap_or("./logs_server".to_string());

    let state = AppState::new(&mongoldb_connection_string, &log_path).await;

    let app = handlers::new(state);

    let listener = TcpListener::bind(api_socket).await.unwrap();

    println!("{:<12} - {:?}", "Listening", listener.local_addr());

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

    Ok(())
}
