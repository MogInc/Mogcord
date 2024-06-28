use dotenv::dotenv;
use serde_json::json;
use uuid::Uuid;
use std::{env, sync::Arc};

use axum::{http::{Method, StatusCode, Uri}, middleware, response::{IntoResponse, Response}, routing::Router, Json};
use mogcord::{api::{chat::routes_chat, user::routes_user}, db::mongoldb::MongolDB, model::{chat::ChatRepository, message::MessageRepository, misc::{log_request, AppState, ServerError}, user::UserRepository}};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    dotenv().ok();

    let mongoldb_connection_string = env::var("MONGOLDB_CONNECTION")
        .unwrap_or("mongodb://localhost:27017".to_owned());

    let api_socket = env::var("API_SOCKET")
        .unwrap_or("127.0.0.1:3000".to_owned());

    let db = Arc::new(MongolDB::init(&mongoldb_connection_string).await?);
    
    let repo_user = Arc::clone(&db) as Arc<dyn UserRepository>;
    let repo_chat =  Arc::clone(&db) as Arc<dyn ChatRepository>;
    let repo_message = Arc::clone(&db) as Arc<dyn MessageRepository>;

    let state: Arc<AppState> = Arc::new(AppState {
        repo_chat,
        repo_user,
        repo_message,
    });

    let api_routes = Router::new()
        .merge(routes_user(state.clone()))
        .merge(routes_chat(state.clone()));


    let app: Router = Router::new()
        .nest("/api", api_routes)
        .layer(middleware::map_response(main_response_mapper))
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

async fn main_response_mapper(
	uri: Uri,
	req_method: Method,
	res: Response
) -> Response 
{
	let req_id = Uuid::now_v7();

	let service_error = res
        .extensions()
        .get::<ServerError>();
	let client_status_error = service_error
        .map(|se| se.client_status_and_error());

	let error_response =
		client_status_error
			.as_ref()
			.map(|(status_code, client_error)| {
				let client_error_body = json!({
					"error": {
                        "req_id": req_id.to_string(),
						"type": client_error.as_ref(),
					}
				});
        
				(*status_code, Json(client_error_body)).into_response()
			});
    
    let client_error = client_status_error.unzip().1;
    log_request(req_id, req_method, uri, service_error, client_error).await;

	println!();
	error_response.unwrap_or(res)
}