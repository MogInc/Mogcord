use axum::async_trait;

use crate::model::error;

use super::RefreshToken;


#[async_trait]
pub trait Repository: Send + Sync 
{
    async fn create_token<'input, 'stack>(&'input self, token: RefreshToken) -> Result<RefreshToken, error::Server<'stack>>;
    async fn get_valid_token_by_device_id<'input, 'stack>(&'input self, device_id: &'input str) -> Result<RefreshToken, error::Server<'stack>>;
    async fn revoke_token<'input, 'stack>(&'input self, user_id: &'input str, device_id: &'input str) -> Result<(), error::Server<'stack>>;
    async fn revoke_all_tokens<'input, 'stack>(&'input self, user_id: &'input str) -> Result<(), error::Server<'stack>>;
}