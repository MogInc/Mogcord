use std::sync::Arc;
use axum::{extract::{self, Path, State}, response::IntoResponse, routing::{get, post, Router}, Json};
use mongodb::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use derive_more::{Display};

#[path ="../model/user/mod.rs"]
mod model;
use model::{user::User, user_error::UserError};

pub fn routes_user(state: Arc<Client>) -> Router
{
    Router::new()
    .route("/user/:id", get(get_user))
    .route("/user", post(post_user))
    .with_state(state)
}

const DB_NAME: &str = "user";

async fn get_user(Path(uuid): Path<String>) 
-> impl IntoResponse
{
    println!("{}", uuid);
}

#[derive(Deserialize)]
pub struct CreateUserRequest
{
    user_name: String,
    user_mail: String,
}

async fn post_user(
    State(client): State<Arc<Client>>, 
    extract::Json(payload): extract::Json<CreateUserRequest>) 
    -> impl IntoResponse
{
    let accounts = client.database(DB_NAME).collection::<User>("accounts");
    
    let account = User::new(payload.user_name, payload.user_mail);

    let _ = accounts.insert_one(&account, None).await;

    return Json(account);
}