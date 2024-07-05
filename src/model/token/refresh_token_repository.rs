use axum::async_trait;

use crate::model::misc::ServerError;

use super::RefreshToken;


#[async_trait]
pub trait RefreshTokenRepository: Send + Sync 
{
    async fn get_token_by_device_id(&self, device_id: &str) -> Result<RefreshToken, ServerError>;
}