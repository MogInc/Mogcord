use std::sync::Arc;
use serde::Deserialize;

use crate::model::{error, user::User, AppState, Hashing};
use crate::server_error;


#[derive(Deserialize)]
pub struct CreateUserRequest
{
    username: String,
    email: String,
    password: String,
}

impl CreateUserRequest
{
    #[must_use]
    pub fn new(username: String, email: String, password: String) -> Self
    {
        Self
        {
            username,
            email,
            password,
        }
    }
}

pub async fn create_user<'a, 'err>(
    state: &'a Arc<AppState>, 
    payload: &'a CreateUserRequest,
) -> error::Result<'err, User>
{
    let repo_user = &state.users;

    //TODO: add user ban checks
    //TODO: email verification (never)

    if repo_user.does_user_exist_by_username(&payload.username).await?
    {
        return Err(server_error!(error::Kind::AlreadyInUse, error::OnType::Username)
            .add_client(error::Client::USERNAME_IN_USE)
        );
    }

    if repo_user.does_user_exist_by_mail(&payload.email).await?
    {
        return Err(server_error!(error::Kind::AlreadyInUse, error::OnType::Email)
            .add_client(error::Client::MAIL_IN_USE)
        );
    }

    let hashed_password = Hashing::hash_text(&payload.password).await?;

    let user = User::new(payload.username.to_string(), payload.email.to_string(), hashed_password);

    match repo_user.create_user(user).await 
    {
        Ok(user) => Ok(user),
        Err(e) => Err(e),
    }
}