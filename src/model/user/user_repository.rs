use axum::async_trait;

use crate::model::misc::{ServerError, Pagination};

use super::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync 
{
    async fn does_user_exist_by_id(&self, user_id: &String) -> Result<bool, ServerError>;
    async fn does_user_exist_by_mail(&self, user_mail: &String) -> Result<bool, ServerError>;
    async fn create_user(&self, user: User) -> Result<User, ServerError>;
    async fn get_user_by_id(&self, user_id: &String) -> Result<User, ServerError>;
    async fn get_users(&self, pagination: Pagination) -> Result<Vec<User>, ServerError>;
    async fn get_users_by_ids(&self, user_ids: Vec<String>) -> Result<Vec<User>, ServerError>;
}