use axum::async_trait;

use super::{chat, server};

#[async_trait]
pub trait Repository: Send + Sync + server::Repository + chat::Repository
{

}