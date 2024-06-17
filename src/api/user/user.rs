use std::sync::Arc;
use axum::{extract::{self, Path, State}, response::IntoResponse, routing::{get, post, Router}, Json};
use serde::Deserialize;

use crate::model::user::{UserError, UserRepository};
use crate::{db::mongoldb::MongolDB, model::user::User};

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
    State(db): State<Arc<dyn UserRepository>>, 
    extract::Json(payload): extract::Json<CreateUserRequest>) 
    -> impl IntoResponse
{
    let user = User::new(payload.user_name, payload.user_mail);

    if db.does_user_exist_by_mail(&user.user_mail).await?
    {
        return Err(UserError::MailAlreadyInUse);
    }

    match db.create_user(user).await 
    {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e),
    }
}