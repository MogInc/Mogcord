use std::sync::Arc;
use axum::{extract::{Path, State}, response::IntoResponse, routing::{get, post, Router}, Json};
use mongodb::Client;
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

async fn post_user(State(client): State<Arc<Client>>) 
{
    let accounts = client.database("chat").collection::<User>("accounts");

    let account = User::new(String::from("Name"), String::from("mail"));

    accounts.insert_one(account, None).await;
}