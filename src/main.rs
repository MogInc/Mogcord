use dotenv::dotenv;
use mogcord::io::FileWriter;
use mogcord::middleware::auth::mw_ctx_resolver;
use tower_cookies::CookieManagerLayer;
use std::{env, sync::Arc};
use axum::{http::StatusCode, middleware, response::IntoResponse, routing::Router};
use tokio::net::TcpListener;

use mogcord::model::{log, AppState};
use mogcord::handlers;
use mogcord::middleware::logging::api_response_mapper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    dotenv().ok();

    let mongoldb_connection_string = env::var("MONGOLDB_CONNECTION")
        .unwrap_or("mongodb://localhost:27017".to_string());

    let api_socket = env::var("API_SOCKET")
        .unwrap_or("127.0.0.1:3000".to_string());

    let log_path = env::var("LOG_PATH")
        .unwrap_or("./logs_server".to_string());

    let state = AppState::new(&mongoldb_connection_string).await;
    
    
    let logs = Arc::new(FileWriter::new(log_path)) as Arc<dyn log::Repository>;


    let app: Router = Router::new()
        .nest("/", handlers::web::routes(state.clone()))
        .nest("/api", handlers::api::routes(state.clone()))
        .fallback(page_not_found);


    let listener: TcpListener = TcpListener::bind(api_socket)
        .await
        .unwrap();


    println!("{:<12} - {:?}", "Listening", listener.local_addr());


    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn page_not_found() -> impl IntoResponse 
{
    (StatusCode::NOT_FOUND, "404 Page Not Found")
}