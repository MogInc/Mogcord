use axum::async_trait;

use crate::model::{error, pagination::Pagination};
use super::User;


#[async_trait]
pub trait Repository: Send + Sync 
{
    async fn does_user_exist_by_id<'input, 'err>(&'input self, user_id: &'input str) -> error::Result<'err, bool>;
    async fn does_user_exist_by_mail<'input, 'err>(&'input self, user_mail: &'input str) -> error::Result<'err, bool>;
    async fn does_user_exist_by_username<'input, 'err>(&'input self, username: &'input str) -> error::Result<'err, bool>;
    async fn create_user<'input, 'err>(&'input self, user: User) -> error::Result<'err, User>;
    async fn create_users<'input, 'err>(&'input self, users: Vec<User>) -> error::Result<'err, ()>;
    async fn get_user_by_id<'input, 'err>(&'input self, user_id: &'input str) -> error::Result<'err, User>;
    async fn get_user_by_mail<'input, 'err>(&'input self, mail: &'input str) -> error::Result<'err, User>;
    async fn get_users_by_id<'input, 'err>(&'input self, user_ids: Vec<String>) -> error::Result<'err, Vec<User>>;
    async fn get_users<'input, 'err>(&'input self, pagination: Pagination) -> error::Result<'err, Vec<User>>;
}