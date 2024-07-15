use axum::async_trait;

use crate::model::misc::{ServerError, Pagination};

use super::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync 
{
    async fn does_valid_user_exist_by_id(&self, user_id: &str) -> Result<bool, ServerError>;
    async fn does_valid_user_exist_by_mail(&self, user_mail: &str) -> Result<bool, ServerError>;
    async fn does_valid_user_exist_by_username(&self, username: &str) -> Result<bool, ServerError>;
    async fn create_user(&self, user: User) -> Result<User, ServerError>;
    async fn create_users(&self, users: Vec<User>) -> Result<(), ServerError>;
    async fn get_valid_user_by_id(&self, user_id: &str) -> Result<User, ServerError>;
    async fn get_valid_user_by_mail(&self, mail: &str) -> Result<User, ServerError>;
    async fn get_valid_users_by_id(&self, user_ids: Vec<String>) -> Result<Vec<User>, ServerError>;
    async fn get_all_users(&self, pagination: Pagination) -> Result<Vec<User>, ServerError>;
}