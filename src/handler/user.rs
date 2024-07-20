use std::sync::Arc;
use axum::{extract::{Path, Query, State}, response::IntoResponse, Json};
use serde::Deserialize;

use crate::model::{error, AppState, Hashing, Pagination};
use crate::middleware::auth::Ctx;
use crate::dto::{vec_to_dto, ObjectToDTO, UserCreateResponse, UserGetResponse};
use crate::model::user::User;


pub async fn get_user_for_admin(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>
) -> impl IntoResponse
{   
    let repo_user = &state.user;

    match repo_user.get_user_by_id(&user_id).await 
    {
        Ok(user) => Ok(Json(UserGetResponse::obj_to_dto(user))),
        Err(e) => Err(e),
    }
}

pub async fn get_users_for_admin(
    State(state): State<Arc<AppState>>,
    pagination: Option<Query<Pagination>>,
) -> impl IntoResponse
{
    let repo_user = &state.user;

    let pagination = Pagination::new(pagination);

    match repo_user.get_users(pagination).await 
    {
        Ok(users) => Ok(Json(vec_to_dto::<User, UserGetResponse>(users))),
        Err(e) => Err(e),
    }
}

pub async fn get_ctx_user_for_authenticated(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
) -> impl IntoResponse
{
    let repo_user = &state.user;
 
    let ctx_user_id = &ctx.user_id_ref();
    
    match repo_user.get_user_by_id(ctx_user_id).await 
    {
        Ok(user) => Ok(Json(UserGetResponse::obj_to_dto(user))),
        Err(e) => Err(e),
    }
}


#[derive(Deserialize)]
pub struct CreateUserRequest
{
    username: String,
    mail: String,
    password: String,
}

pub async fn create_user_for_everyone(
    State(state): State<Arc<AppState>>, 
    Json(payload): Json<CreateUserRequest>
) -> impl IntoResponse
{
    let repo_user = &state.user;

    //TODO: add user ban checks
    //TODO: mail verification (never)

    if repo_user.does_user_exist_by_username(&payload.username).await?
    {
        return Err(error::Server::UsernameAlreadyInUse);
    }

    if repo_user.does_user_exist_by_mail(&payload.mail).await?
    {
        return Err(error::Server::MailAlreadyInUse);
    }

    let hashed_password = Hashing::hash_text(&payload.password).await?;

    let user = User::new(payload.username, payload.mail, hashed_password);


    match repo_user.create_user(user).await 
    {
        Ok(user) => Ok(Json(UserCreateResponse::obj_to_dto(user))),
        Err(e) => Err(e),
    }
}