use axum::async_trait;

use crate::model::error::ServerError;

use super::RefreshToken;


#[async_trait]
pub trait RefreshTokenRepository: Send + Sync 
{
    async fn create_token(&self, token: RefreshToken) -> Result<RefreshToken, ServerError>;
    async fn get_valid_token_by_device_id(&self, device_id: &str) -> Result<RefreshToken, ServerError>;
    async fn revoke_token(&self, user_id: &str, device_id: &str) -> Result<(), ServerError>;
    async fn revoke_all_tokens(&self, user_id: &str) -> Result<(), ServerError>;
}