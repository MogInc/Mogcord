use std::sync::Arc;
use axum::{extract::{Path, Query, State}, middleware, response::IntoResponse, routing::{get, post, Router}, Json};
use serde::Deserialize;

use crate::{dto::UserDTO, middleware::auth::{self, Ctx}, model::misc::{AppState, Hashing, Pagination, ServerError}};
use crate::model::user::User;

pub fn routes_user(state: Arc<AppState>) -> Router
{
    let routes_with_regular_middleware = Router::new()
        .route("/user", get(get_ctx_user_for_authenticated))
        .with_state(state.clone())
        .route_layer(middleware::from_fn(auth::mw_require_regular_auth))
        .route_layer(middleware::from_fn(auth::mw_ctx_resolver));

    let routes_with_admin_middleware = Router::new()
        .route("/admin/user/:user_id", get(get_user_for_admin))
        .route("/admin/users", get(get_users_for_admin))
        .with_state(state.clone())
        .route_layer(middleware::from_fn(auth::mw_require_management_auth))
        .route_layer(middleware::from_fn(auth::mw_ctx_resolver));


    let routes_without_middleware = Router::new()
        .route("/user", post(create_user_for_everyone))
        .with_state(state);

    Router::new()
        .merge(routes_with_regular_middleware)
        .merge(routes_with_admin_middleware)
        .merge(routes_without_middleware)
}


async fn get_user_for_admin(
    State(state): State<Arc<AppState>>,
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

async fn get_users_for_admin(
    State(state): State<Arc<AppState>>,
    pagination: Option<Query<Pagination>>,
) -> impl IntoResponse
{
    let repo_user = &state.repo_user;

    let pagination = Pagination::new(pagination);

    match repo_user.get_all_users(pagination).await 
    {
        Ok(users) => Ok(Json(UserDTO::vec_to_dto(users))),
        Err(e) => Err(e),
    }
}

async fn get_ctx_user_for_authenticated(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
) -> impl IntoResponse
{
    let repo_user = &state.repo_user;
 
    let ctx_user_id = ctx.user_id_ref();
    
    match repo_user.get_user_by_id(&ctx_user_id).await 
    {
        Ok(user) => Ok(Json(UserDTO::obj_to_dto(user))),
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

async fn create_user_for_everyone(
    State(state): State<Arc<AppState>>, 
    Json(payload): Json<CreateUserRequest>
) -> impl IntoResponse
{
    let repo_user = &state.repo_user;

    //TODO: add user ban checks
    //TODO: mail verification (never)

    if repo_user.does_user_exist_by_username(&payload.username).await?
    {
        return Err(ServerError::UsernameAlreadyInUse);
    }

    if repo_user.does_user_exist_by_mail(&payload.mail).await?
    {
        return Err(ServerError::MailAlreadyInUse);
    }

    let hashed_password = Hashing::hash_text(&payload.password).await?;

    let user = User::new(payload.username, payload.mail, hashed_password);


    match repo_user.create_user(user).await 
    {
        Ok(user) => Ok(Json(UserDTO::obj_to_dto(user))),
        Err(e) => Err(e),
    }
}