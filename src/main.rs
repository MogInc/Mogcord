use dotenv::dotenv;
use mogcord::middleware::auth::mw_ctx_resolver;
use tower_cookies::CookieManagerLayer;
use std::{env, sync::Arc};
use axum::{http::StatusCode, middleware, response::IntoResponse, routing::Router};
use tokio::net::TcpListener;

use mogcord::model::{chat, message, refresh_token, relation, server, user, AppState};
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
    
    let user = Arc::clone(&db) as Arc<dyn user::Repository>;
    let chat =  Arc::clone(&db) as Arc<dyn chat::Repository>;
    let server =  Arc::clone(&db) as Arc<dyn server::Repository>;
    let message = Arc::clone(&db) as Arc<dyn message::Repository>;
    let refresh_token = Arc::clone(&db) as Arc<dyn refresh_token::Repository>;
    let relation = Arc::clone(&db) as Arc<dyn relation::Repository>;

    let state: Arc<AppState> = Arc::new(
        AppState 
        {
            chat,
            server,
            user,
            message,
            refresh_token,
            relation,
        }
    );

    let app: Router = Router::new()
        .nest("/api", handler::routes(state))
        .layer(middleware::map_response(main_response_mapper))
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