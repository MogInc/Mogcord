use std::sync::Arc;
use axum::{extract::{self, Path, State}, response::IntoResponse, routing::{get, post, Router}, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use derive_more::{Display};

use crate::model::user::user_repository::UserRepository;
use crate::{db::mongoldb::mongoldb::MongolDB, model::user::user::User};

pub fn routes_user(state: Arc<MongolDB>) -> Router
{
    Router::new()
    .route("/user/:id", get(get_user))
    .route("/user", post(post_user))
    .with_state(state)
}

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
    State(db): State<Arc<MongolDB>>, 
    extract::Json(payload): extract::Json<CreateUserRequest>) 
    -> impl IntoResponse
{
    let user = User::new(payload.user_name, payload.user_mail);

    match db.create_user(user).await 
    {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e),
    }
}