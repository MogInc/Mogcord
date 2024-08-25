use axum::async_trait;

use crate::model::error;

#[async_trait]
pub trait Repository: Send + Sync
{
    async fn does_friendship_exist<'input, 'err>(
        &'input self,
        current_user_id: &'input str,
        other_user_id: &'input str,
    ) -> error::Result<'err, bool>;
    async fn does_friendships_exist<'input, 'err>(
        &'input self,
        current_user_id: &'input str,
        other_user_ids: Vec<&'input str>,
    ) -> error::Result<'err, bool>;
    async fn does_incoming_friendship_exist<'input, 'err>(
        &'input self,
        current_user_id: &'input str,
        other_user_id: &'input str,
    ) -> error::Result<'err, bool>;
    async fn does_outgoing_friendship_exist<'input, 'err>(
        &'input self,
        current_user_id: &'input str,
        other_user_id: &'input str,
    ) -> error::Result<'err, bool>;
    async fn add_user_as_friend<'input, 'err>(
        &'input self,
        current_user_id: &'input str,
        other_user_id: &'input str,
    ) -> error::Result<'err, ()>;
    async fn confirm_user_as_friend<'input, 'err>(
        &'input self,
        current_user_id: &'input str,
        other_user_id: &'input str,
    ) -> error::Result<'err, ()>;
    async fn remove_user_as_friend<'input, 'err>(
        &'input self,
        current_user_id: &'input str,
        other_user_id: &'input str,
    ) -> error::Result<'err, ()>;
    async fn does_blocked_exist<'input, 'err>(
        &'input self,
        current_user_id: &'input str,
        other_user_id: &'input str,
    ) -> error::Result<'err, bool>;
    async fn add_user_as_blocked<'input, 'err>(
        &'input self,
        current_user_id: &'input str,
        other_user_id: &'input str,
    ) -> error::Result<'err, ()>;
    async fn remove_user_as_blocked<'input, 'err>(
        &'input self,
        current_user_id: &'input str,
        other_user_id: &'input str,
    ) -> error::Result<'err, ()>;
}
