use std::sync::Arc;
use axum::{extract::{Path, Query, State}, middleware, response::IntoResponse, routing::{get, post, Router}, Json};
use serde::Deserialize;

use crate::{dto::UserDTO, middleware::Ctx, model::misc::{AppState, Hashing, Pagination, ServerError}};
use crate::model::user::User;
use crate::middleware as mw;

pub fn routes_user(state: Arc<AppState>) -> Router
{
    let routes_with_middleware = Router::new()
        .route("/user", get(get_current_user))
        .route("/user/:user_id", get(get_user))
        .route("/users", get(get_users))
        .with_state(state.clone())
        .route_layer(middleware::from_fn(mw::mw_require_auth))
        .route_layer(middleware::from_fn(mw::mw_ctx_resolver));

    let routes_without_middleware = Router::new()
        .route("/user", post(create_user))
        .with_state(state);

    return Router::new()
        .merge(routes_with_middleware)
        .merge(routes_without_middleware);
}


async fn get_user(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Path(user_id): Path<String>
) -> impl IntoResponse
{
    let repo_user = &state.repo_user;

    match repo_user.get_user_by_id(&user_id).await 
    {
        Ok(user) => Ok(Json(UserDTO::obj_to_dto(user))),
        Err(e) => Err(e),
    }
}

async fn get_current_user(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
) -> impl IntoResponse
{
    let current_user_id = ctx.user_id();

    let repo_user = &state.repo_user;

    match repo_user.get_user_by_id(&current_user_id).await 
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
    username: String,
    mail: String,
    password: String,
}

async fn create_user(
    State(state): State<Arc<AppState>>, 
    Json(payload): Json<CreateUserRequest>
) -> impl IntoResponse
{
    let repo_user = &state.repo_user;

    let hashed_password = Hashing::hash_text(&payload.password).await?;

    let user = User::new(payload.username, payload.mail, hashed_password);

    if repo_user.does_username_exist(&user.username).await?
    {
        return Err(ServerError::UsernameAlreadyInUse);
    }

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