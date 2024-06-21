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

async fn get_user(
    State(repo): State<Arc<dyn UserRepository>>,
    Path(uuid): Path<String>) 
    -> impl IntoResponse
{
    match repo.get_user_by_id(&uuid).await 
    {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e),
    }
}

#[derive(Deserialize)]
struct CreateUserRequest
{
    user_name: String,
    user_mail: String,
}

async fn post_user(
    State(repo): State<Arc<dyn UserRepository>>, 
    extract::Json(payload): extract::Json<CreateUserRequest>) 
    -> impl IntoResponse
{

    let user: User = User::new(payload.user_name, payload.user_mail);

    if repo.does_user_exist_by_mail(&user.mail).await?
    {
        return Err(UserError::MailAlreadyInUse);
    }

    match repo.create_user(user).await 
    {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e),
    }
}