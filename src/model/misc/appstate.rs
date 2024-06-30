use std::sync::Arc;

use crate::model::{chat::ChatRepository, message::MessageRepository, user::UserRepository};

pub struct AppState {
    pub repo_chat: Arc<dyn ChatRepository>,
    pub repo_user: Arc<dyn UserRepository>,
    pub repo_message: Arc<dyn MessageRepository>,
}