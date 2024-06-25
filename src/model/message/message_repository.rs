use axum::async_trait;

use crate::model::misc::{Pagination, ServerError};

use super::Message;

#[async_trait]
pub trait MessageRepository: Send + Sync
{
    async fn get_messages(&self, chat_id: &String, pagination: Pagination) -> Result<Vec<Message>, ServerError>;
}