use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::model::{error, user::User, AppState, Hashing};
use crate::dto::{ObjectToDTO, UserCreateResponse};
use crate::server_error;


#[derive(Deserialize)]
pub struct CreateUserRequest
{
    username: String,
    mail: String,
    password: String,
}

pub async fn create_user(
    State(state): State<Arc<AppState>>, 
    Json(payload): Json<CreateUserRequest>
) -> impl IntoResponse
{
    let repo_user = &state.users;

    //TODO: add user ban checks
    //TODO: mail verification (never)

    if repo_user.does_user_exist_by_username(&payload.username).await?
    {
        return Err(server_error!(error::Kind::AlreadyInUse, error::OnType::Username)
            .add_client(error::Client::USERNAME_IN_USE)
        );
    }

    if repo_user.does_user_exist_by_mail(&payload.mail).await?
    {
        return Err(server_error!(error::Kind::AlreadyInUse, error::OnType::Mail)
            .add_client(error::Client::MAIL_IN_USE)
        );
    }

    let hashed_password = Hashing::hash_text(&payload.password).await?;

    let user = User::new(payload.username, payload.mail, hashed_password);


    match repo_user.create_user(user).await 
    {
        Ok(user) => Ok(Json(UserCreateResponse::obj_to_dto(user))),
        Err(e) => Err(e),
    }
}