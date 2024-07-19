use axum::async_trait;

use crate::model::error;

use super::RefreshToken;


#[async_trait]
pub trait Repository: Send + Sync 
{
    async fn create_token(&self, token: RefreshToken) -> Result<RefreshToken, error::Server>;
    async fn get_valid_token_by_device_id(&self, device_id: &str) -> Result<RefreshToken, error::Server>;
    async fn revoke_token(&self, user_id: &str, device_id: &str) -> Result<(), error::Server>;
    async fn revoke_all_tokens(&self, user_id: &str) -> Result<(), error::Server>;
}