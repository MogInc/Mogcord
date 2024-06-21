use std::sync::Arc;

use crate::model::{chat::ChatRepository, user::UserRepository};

pub struct AppState {
    pub repo_chat: Arc<dyn ChatRepository>,
    pub repo_user: Arc<dyn UserRepository>,
}