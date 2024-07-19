use axum::async_trait;

use crate::model::error;


#[async_trait]
pub trait Repository: Send + Sync
{
    async fn does_friendship_exist(&self, current_user_id: &str, other_user_id: &str) -> Result<bool, error::Server>;
    async fn does_incoming_friendship_exist(&self, current_user_id: &str, other_user_id: &str) -> Result<bool, error::Server>;
    async fn add_user_as_friend(&self, current_user_id: &str, other_user_id: &str) -> Result<(), error::Server>;
    async fn confirm_user_as_friend(&self, current_user_id: &str, other_user_id: &str) -> Result<(), error::Server>;
    async fn remove_user_as_friend(&self, current_user_id: &str, other_user_id: &str) -> Result<(), error::Server>;
    async fn does_blocked_exist(&self, current_user_id: &str, other_user_id: &str) -> Result<bool, error::Server>;
    async fn add_user_as_blocked(&self, current_user_id: &str, other_user_id: &str) -> Result<(), error::Server>;
    async fn remove_user_as_blocked(&self, current_user_id: &str, other_user_id: &str) -> Result<(), error::Server>;
}