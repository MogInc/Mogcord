use axum::async_trait;

use crate::{db::mongoldb::MongolDB, model::{misc::ServerError, token::{RefreshToken, RefreshTokenRepository}}};

#[async_trait]
impl RefreshTokenRepository for MongolDB
{
    async fn get_token_by_device_id(&self, device_id: &str) -> Result<RefreshToken, ServerError>
    {
        
    }
}