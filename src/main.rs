use dotenv::dotenv;
use std::{env, sync::Arc};

use axum::{http::StatusCode, response::IntoResponse, routing::Router};
use mogcord::{api::{chat::routes_chat, user::routes_user}, db::mongoldb::MongolDB, model::{appstate::AppState, chat::ChatRepository, user::UserRepository}};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() 
    -> Result<(), Box<dyn std::error::Error>> 
{
    dotenv().ok();

    let mongoldb_connection_string = env::var("MONGOLDB_CONNECTION")
        .unwrap_or("mongodb://localhost:27017".to_owned());

    let api_socket = env::var("API_SOCKET")
        .unwrap_or("127.0.0.1:3000".to_owned());

    println!("{}", mongoldb_connection_string);

    let db: MongolDB = MongolDB::init(&mongoldb_connection_string).await?;
    
    let repo_chat: Arc<dyn ChatRepository> = Arc::new(db.clone());
    let repo_user: Arc<dyn UserRepository> = Arc::new(db.clone());

    let state: Arc<AppState> = Arc::new(AppState {
        repo_chat,
        repo_user,
    });

    let api_routes = Router::new()
        .merge(routes_user(state.clone()))
        .merge(routes_chat(state.clone()));


    let app: Router = Router::new()
        .nest("/api", api_routes)
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