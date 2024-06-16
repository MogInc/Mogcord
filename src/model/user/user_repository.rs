use axum::async_trait;

use super::{user::User, user_error::UserError};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: User) -> Result<User, UserError>;
}