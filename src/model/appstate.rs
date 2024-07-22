use std::sync::Arc;

use super::{channel, chat, message, refresh_token, relation, server, user};


pub struct AppState 
{
    pub chats: Arc<dyn chat::Repository>,
    pub servers: Arc<dyn server::Repository>,
    pub channels: Arc<dyn channel::Repository>,
    pub users: Arc<dyn user::Repository>,
    pub messages: Arc<dyn message::Repository>,
    pub refresh_tokens: Arc<dyn refresh_token::Repository>,
    pub relations: Arc<dyn relation::Repository>,
}