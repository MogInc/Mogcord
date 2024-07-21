use axum::async_trait;

#[async_trait]
pub trait Repository: Send + Sync
{

}