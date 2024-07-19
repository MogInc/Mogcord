use std::sync::Arc;

use super::{chat, message, refresh_token::RefreshTokenRepository, relation::RelationRepository, user::UserRepository};


pub struct AppState 
{
    pub chat: Arc<dyn chat::Repository>,
    pub user: Arc<dyn UserRepository>,
    pub message: Arc<dyn message::Repository>,
    pub refresh_token: Arc<dyn RefreshTokenRepository>,
    pub relation: Arc<dyn RelationRepository>,
}