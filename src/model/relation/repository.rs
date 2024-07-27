use axum::async_trait;

use crate::model::error;


#[async_trait]
pub trait Repository: Send + Sync
{
    async fn does_friendship_exist<'input, 'stack>(&'input self, current_user_id: &'input str, other_user_id: &'input str) -> Result<bool, error::Server<'stack>>;
    async fn does_friendships_exist<'input, 'stack>(&'input self, current_user_id: &'input str, other_user_ids: Vec<&'input str>) -> Result<bool, error::Server<'stack>>;
    async fn does_incoming_friendship_exist<'input, 'stack>(&'input self, current_user_id: &'input str, other_user_id: &'input str) -> Result<bool, error::Server<'stack>>;
    async fn does_outgoing_friendship_exist<'input, 'stack>(&'input self, current_user_id: &'input str, other_user_id: &'input str) -> Result<bool, error::Server<'stack>>;
    async fn add_user_as_friend<'input, 'stack>(&'input self, current_user_id: &'input str, other_user_id: &'input str) -> Result<(), error::Server<'stack>>;
    async fn confirm_user_as_friend<'input, 'stack>(&'input self, current_user_id: &'input str, other_user_id: &'input str) -> Result<(), error::Server<'stack>>;
    async fn remove_user_as_friend<'input, 'stack>(&'input self, current_user_id: &'input str, other_user_id: &'input str) -> Result<(), error::Server<'stack>>;
    async fn does_blocked_exist<'input, 'stack>(&'input self, current_user_id: &'input str, other_user_id: &'input str) -> Result<bool, error::Server<'stack>>;
    async fn add_user_as_blocked<'input, 'stack>(&'input self, current_user_id: &'input str, other_user_id: &'input str) -> Result<(), error::Server<'stack>>;
    async fn remove_user_as_blocked<'input, 'stack>(&'input self, current_user_id: &'input str, other_user_id: &'input str) -> Result<(), error::Server<'stack>>;
}