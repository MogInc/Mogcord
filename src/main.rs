use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::Router};
use mogcord::{api::{chat::routes_chat, user::routes_user}, db::mongoldb::MongolDB, model::{appstate::AppState, chat::ChatRepository, user::UserRepository}};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() 
    -> Result<(), Box<dyn std::error::Error>> 
{
    let mongodb_address = "mongodb://localhost:27017";
    let address: &str = "127.0.0.1:8080";

    let db: MongolDB = MongolDB::init(mongodb_address).await?;
    
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


    let listener: TcpListener = TcpListener::bind(address)
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