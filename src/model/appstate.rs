use std::sync::Arc;

use super::{chat::ChatRepository, message::MessageRepository, refresh_token::RefreshTokenRepository, relation::RelationRepository, user::UserRepository};


pub struct AppState 
{
    pub chat: Arc<dyn ChatRepository>,
    pub user: Arc<dyn UserRepository>,
    pub message: Arc<dyn MessageRepository>,
    pub refresh_token: Arc<dyn RefreshTokenRepository>,
    pub relation: Arc<dyn RelationRepository>,
}