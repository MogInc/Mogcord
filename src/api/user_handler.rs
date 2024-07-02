use std::sync::Arc;
use axum::{extract::{self, Path, Query, State}, response::IntoResponse, routing::{get, post, Router}, Json};
use serde::Deserialize;

use crate::{dto::UserDTO, middleware::Ctx, model::misc::{AppState, Pagination, ServerError}};
use crate::model::user::User;

pub fn routes_user(state: Arc<AppState>) -> Router
{
    Router::new()
    .route("/user", post(create_user))
    .route("/user/:user_id", get(get_user))
    .route("/users", get(get_users))
    .with_state(state)
}

async fn get_user(
    ctx: Ctx,
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> impl IntoResponse
{
    //TODO: Add AA
    println!("{ctx:?}");


    let repo_user = &state.repo_user;

    match repo_user.get_user_by_id(&user_id).await 
    {
        Ok(user) => Ok(Json(UserDTO::obj_to_dto(user))),
        Err(e) => Err(e),
    }
}


async fn get_users(
    State(state): State<Arc<AppState>>,
    pagination: Option<Query<Pagination>>,
) -> impl IntoResponse
{
    //TODO: Add AA

    let repo_user = &state.repo_user;

    let pagination = Pagination::new(pagination);

    match repo_user.get_users(pagination).await 
    {
        Ok(users) => Ok(Json(UserDTO::vec_to_dto(users))),
        Err(e) => Err(e),
    }
}

#[derive(Deserialize)]
struct CreateUserRequest
{
    user_name: String,
    user_mail: String,
}

async fn create_user(
    State(state): State<Arc<AppState>>, 
    extract::Json(payload): extract::Json<CreateUserRequest>
) -> impl IntoResponse
{
    let repo_user = &state.repo_user;

    let user = User::new(payload.user_name, payload.user_mail);

    if repo_user.does_user_exist_by_mail(&user.mail).await?
    {
        return Err(ServerError::MailAlreadyInUse);
    }

    match repo_user.create_user(user).await 
    {
        Ok(user) => Ok(Json(UserDTO::obj_to_dto(user))),
        Err(e) => Err(e),
    }
}