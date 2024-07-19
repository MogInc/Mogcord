use dotenv::dotenv;
use tower_cookies::CookieManagerLayer;
use std::{env, sync::Arc};
use axum::{http::StatusCode, middleware, response::IntoResponse, routing::Router};
use tokio::net::TcpListener;

use mogcord::model::{chat, message, AppState, relation::RelationRepository, refresh_token::RefreshTokenRepository, user::UserRepository};
use mogcord::handler;
use mogcord::middleware::logging::main_response_mapper;
use mogcord::db::MongolDB;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    dotenv().ok();

    let mongoldb_connection_string = env::var("MONGOLDB_CONNECTION")
        .unwrap_or("mongodb://localhost:27017".to_owned());

    let api_socket = env::var("API_SOCKET")
        .unwrap_or("127.0.0.1:3000".to_owned());

    let db = Arc::new(MongolDB::init(&mongoldb_connection_string).await?);
    
    let user = Arc::clone(&db) as Arc<dyn UserRepository>;
    let chat =  Arc::clone(&db) as Arc<dyn chat::Repository>;
    let message = Arc::clone(&db) as Arc<dyn message::Repository>;
    let refresh_token = Arc::clone(&db) as Arc<dyn RefreshTokenRepository>;
    let relation = Arc::clone(&db) as Arc<dyn RelationRepository>;

    let state: Arc<AppState> = Arc::new(
        AppState 
        {
            chat,
            user,
            message,
            refresh_token,
            relation,
        }
    );

    let api_routes = Router::new()
        .merge(handler::auth::routes(state.clone()))
        .merge(handler::chat::routes(state.clone()))
        .merge(handler::message::routes(state.clone()))
        .merge(handler::relation::routes(state.clone()))
        .merge(handler::user::routes(state));


    let app: Router = Router::new()
        .nest("/api", api_routes)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
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