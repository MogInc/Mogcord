use dotenv::dotenv;
use mogcord::file_writer::FileWriter;
use mogcord::middleware::auth::mw_ctx_resolver;
use tower_cookies::CookieManagerLayer;
use std::{env, sync::Arc};
use axum::{http::StatusCode, middleware, response::IntoResponse, routing::Router};
use tokio::net::TcpListener;

use mogcord::model::{channel, channel_parent, log, message, refresh_token, relation, user, AppState};
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
    
    let chats =  Arc::clone(&db) as Arc<dyn channel_parent::Repository>;
    let servers =  Arc::clone(&db) as Arc<dyn channel_parent::Repository>;
    let channel_parents =  Arc::clone(&db) as Arc<dyn channel_parent::Repository>;
    let channels =  Arc::clone(&db) as Arc<dyn channel::Repository>;
    let users = Arc::clone(&db) as Arc<dyn user::Repository>;
    let messages = Arc::clone(&db) as Arc<dyn message::Repository>;
    let refresh_tokens = Arc::clone(&db) as Arc<dyn refresh_token::Repository>;
    let relations = Arc::clone(&db) as Arc<dyn relation::Repository>;


    let logs = Arc::new(FileWriter::new("./logs_server")) as Arc<dyn log::Repository>;

    let state: Arc<AppState> = Arc::new(
        AppState 
        {
            chats,
            servers,
            channel_parents,
            channels,
            users,
            messages,
            refresh_tokens,
            relations,
        }
    );

    let app: Router = Router::new()
        .nest("/api", handler::routes(state.clone()))
        .layer(middleware::map_response_with_state(logs, main_response_mapper))
        .layer(middleware::from_fn(mw_ctx_resolver))
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