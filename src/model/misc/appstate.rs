use std::sync::Arc;

use crate::model::{chat::ChatRepository, message::MessageRepository, relation::RelationRepository, token::RefreshTokenRepository, user::UserRepository};

pub struct AppState 
{
    pub repo_chat: Arc<dyn ChatRepository>,
    pub repo_user: Arc<dyn UserRepository>,
    pub repo_message: Arc<dyn MessageRepository>,
    pub repo_refresh_token: Arc<dyn RefreshTokenRepository>,
    pub repo_relation: Arc<dyn RelationRepository>,
}