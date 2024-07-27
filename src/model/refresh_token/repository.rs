use axum::async_trait;

use crate::model::error;

use super::RefreshToken;


#[async_trait]
pub trait Repository: Send + Sync 
{
    async fn create_token<'input, 'err>(&'input self, token: RefreshToken) -> Result<RefreshToken, error::Server<'err>>;
    async fn get_valid_token_by_device_id<'input, 'err>(&'input self, device_id: &'input str) -> Result<RefreshToken, error::Server<'err>>;
    async fn revoke_token<'input, 'err>(&'input self, user_id: &'input str, device_id: &'input str) -> Result<(), error::Server<'err>>;
    async fn revoke_all_tokens<'input, 'err>(&'input self, user_id: &'input str) -> Result<(), error::Server<'err>>;
}