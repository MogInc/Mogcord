use axum::async_trait;

use crate::model::{error, pagination::Pagination};
use super::User;


#[async_trait]
pub trait Repository: Send + Sync 
{
    async fn does_user_exist_by_id<'input, 'err>(&'input self, user_id: &'input str) -> Result<bool, error::Server<'err>>;
    async fn does_user_exist_by_mail<'input, 'err>(&'input self, user_mail: &'input str) -> Result<bool, error::Server<'err>>;
    async fn does_user_exist_by_username<'input, 'err>(&'input self, username: &'input str) -> Result<bool, error::Server<'err>>;
    async fn create_user<'input, 'err>(&'input self, user: User) -> Result<User, error::Server<'err>>;
    async fn create_users<'input, 'err>(&'input self, users: Vec<User>) -> Result<(), error::Server<'err>>;
    async fn get_user_by_id<'input, 'err>(&'input self, user_id: &'input str) -> Result<User, error::Server<'err>>;
    async fn get_user_by_mail<'input, 'err>(&'input self, mail: &'input str) -> Result<User, error::Server<'err>>;
    async fn get_users_by_id<'input, 'err>(&'input self, user_ids: Vec<String>) -> Result<Vec<User>, error::Server<'err>>;
    async fn get_users<'input, 'err>(&'input self, pagination: Pagination) -> Result<Vec<User>, error::Server<'err>>;
}