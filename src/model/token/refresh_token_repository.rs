use axum::async_trait;

use crate::model::{misc::ServerError, user::User};

use super::RefreshToken;


#[async_trait]
pub trait RefreshTokenRepository: Send + Sync 
{
    async fn create_token(&self, token: RefreshToken, owner: &User) -> Result<RefreshToken, ServerError>;
    async fn get_token_by_device_id(&self, device_id: &str) -> Result<RefreshToken, ServerError>;
}