use axum::async_trait;

use super::{user::User, user_error::UserError};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn does_user_exist_by_id(&self, user_id: &String) -> Result<bool, UserError>;
    async fn does_user_exist_by_mail(&self, user_mail: &String) -> Result<bool, UserError>;
    async fn create_user(&self, user: User) -> Result<User, UserError>;
}