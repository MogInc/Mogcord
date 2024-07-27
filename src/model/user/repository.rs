use axum::async_trait;

use crate::model::{error, pagination::Pagination};
use super::User;


#[async_trait]
pub trait Repository: Send + Sync 
{
    async fn does_user_exist_by_id<'input, 'stack>(&'input self, user_id: &'input str) -> Result<bool, error::Server<'stack>>;
    async fn does_user_exist_by_mail<'input, 'stack>(&'input self, user_mail: &'input str) -> Result<bool, error::Server<'stack>>;
    async fn does_user_exist_by_username<'input, 'stack>(&'input self, username: &'input str) -> Result<bool, error::Server<'stack>>;
    async fn create_user<'input, 'stack>(&'input self, user: User) -> Result<User, error::Server<'stack>>;
    async fn create_users<'input, 'stack>(&'input self, users: Vec<User>) -> Result<(), error::Server<'stack>>;
    async fn get_user_by_id<'input, 'stack>(&'input self, user_id: &'input str) -> Result<User, error::Server<'stack>>;
    async fn get_user_by_mail<'input, 'stack>(&'input self, mail: &'input str) -> Result<User, error::Server<'stack>>;
    async fn get_users_by_id<'input, 'stack>(&'input self, user_ids: Vec<String>) -> Result<Vec<User>, error::Server<'stack>>;
    async fn get_users<'input, 'stack>(&'input self, pagination: Pagination) -> Result<Vec<User>, error::Server<'stack>>;
}