use std::sync::Arc;

use super::{channel, channel_parent, message, refresh_token, relation, user};


pub struct AppState 
{
    pub chats: Arc<dyn channel_parent::Repository>,
    pub channels: Arc<dyn channel::Repository>,
    pub users: Arc<dyn user::Repository>,
    pub messages: Arc<dyn message::Repository>,
    pub refresh_tokens: Arc<dyn refresh_token::Repository>,
    pub relations: Arc<dyn relation::Repository>,
}