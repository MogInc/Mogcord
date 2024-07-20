use std::sync::Arc;

use super::{chat, message, refresh_token, relation, server, user};


pub struct AppState 
{
    pub chat: Arc<dyn chat::Repository>,
    pub server: Arc<dyn server::Repository>,
    pub user: Arc<dyn user::Repository>,
    pub message: Arc<dyn message::Repository>,
    pub refresh_token: Arc<dyn refresh_token::Repository>,
    pub relation: Arc<dyn relation::Repository>,
}