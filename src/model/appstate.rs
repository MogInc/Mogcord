use std::sync::Arc;

use super::{chat::ChatRepository, message::MessageRepository, refresh_token::RefreshTokenRepository, relation::RelationRepository, user::UserRepository};


pub struct AppState 
{
    pub repo_chat: Arc<dyn ChatRepository>,
    pub repo_user: Arc<dyn UserRepository>,
    pub repo_message: Arc<dyn MessageRepository>,
    pub repo_refresh_token: Arc<dyn RefreshTokenRepository>,
    pub repo_relation: Arc<dyn RelationRepository>,
}