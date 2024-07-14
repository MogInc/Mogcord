use axum::async_trait;

use crate::model::misc::ServerError;

#[async_trait]
pub trait RelationRepository: Send + Sync
{
    async fn does_friendship_exist(&self, current_user_id: &str, other_user_id: &str) -> Result<bool, ServerError>;
    async fn add_friend(&self, current_user_id: &str, other_user_id: &str) -> Result<(), ServerError>;
}