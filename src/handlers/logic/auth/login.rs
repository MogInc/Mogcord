use std::sync::Arc;
use serde::Deserialize;
use tower_cookies::Cookies;


use crate::handlers::logic;
use crate::model::{error, AppState, Hashing};
use crate::server_error;

#[derive(Deserialize)]
pub struct LoginRequest
{
    email: String,
    password: String,
}

impl LoginRequest
{
    #[must_use]
    pub fn new(email: String, password: String) -> Self
    {
        Self
        {
            email,
            password
        }
    }
}

pub async fn login<'err>(
    state: &Arc<AppState>,
    jar: &Cookies,
    ip_addr: String,
    payload: &LoginRequest,
) -> error::Result<'err, ()>
{
    let repo_user = &state.users;

    let user = repo_user
        .get_user_by_mail(&payload.email)
        .await.map_err(|err| 
            server_error!(err).add_client(error::Client::INVALID_PARAMS)
        )?;

    if !user.flag.is_allowed_on_mogcord()
    {
        return Err(server_error!(error::Kind::IncorrectPermissions, error::OnType::User)
            .add_client(error::Client::NOT_ALLOWED_PLATFORM)
            .add_debug_info("user flag", user.flag.to_string())
        );
    }

    Hashing::verify_hash(&payload.password, &user.hashed_password).await.map_err(|err| 
        server_error!(err).add_client(error::Client::INVALID_PARAMS)
    )?;

    let refresh_token = logic::auth::cookies::get_refresh_token(state, jar, ip_addr, user).await?;
    
    logic::auth::cookies::create_auth_cookies(jar, refresh_token)
}