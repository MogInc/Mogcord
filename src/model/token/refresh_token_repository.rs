use axum::async_trait;


#[async_trait]
pub trait RefreshTokenRepository: Send + Sync 
{

}