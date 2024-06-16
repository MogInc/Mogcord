use std::sync::Arc;
use axum::{extract::{self, Path, State}, response::IntoResponse, routing::{get, post, Router}, Json};
use mongodb::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use derive_more::{Display};

#[path ="../model/mod.rs"]
mod model;
use model::user::User;

pub fn routes_user(state: Arc<Client>) -> Router
{
    Router::new()
    .route("/user/:id", get(get_user))
    .route("/user", post(post_user))
    .with_state(state)
}

const DB_NAME: &str = "user";

#[derive(Debug, Display)]
pub enum UserError
{
    UserNotFound,
    MailAlreadyInUse,
    UnexpectedError,
}

async fn get_user(Path(uuid): Path<String>) 
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
{
    let accounts = client.database(DB_NAME).collection::<User>("accounts");
    
    let account = User::new(payload.user_name, payload.user_mail);

    accounts.insert_one(account, None).await;
}