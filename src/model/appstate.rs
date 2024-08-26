use std::sync::Arc;

use crate::db::MongolDB;
use crate::io::FileWriter;

use super::{channel, channel_parent, log, message, refresh_token, relation, user};

pub struct AppState
{
    pub chats: Arc<dyn channel_parent::Repository>,
    pub servers: Arc<dyn channel_parent::Repository>,
    pub channel_parents: Arc<dyn channel_parent::Repository>,
    pub channels: Arc<dyn channel::Repository>,
    pub users: Arc<dyn user::Repository>,
    pub messages: Arc<dyn message::Repository>,
    pub refresh_tokens: Arc<dyn refresh_token::Repository>,
    pub relations: Arc<dyn relation::Repository>,
    pub logs: Arc<dyn log::Repository>,
}

impl AppState
{
    pub async fn new(
        db_con: &str,
        log_path: &str,
    ) -> Arc<Self>
    {
        let db = Arc::new(MongolDB::init(db_con).await.expect("Couldnt connect to db"));

        let chats = Arc::clone(&db) as Arc<dyn channel_parent::Repository>;
        let servers = Arc::clone(&db) as Arc<dyn channel_parent::Repository>;
        let channel_parents = Arc::clone(&db) as Arc<dyn channel_parent::Repository>;
        let channels = Arc::clone(&db) as Arc<dyn channel::Repository>;
        let users = Arc::clone(&db) as Arc<dyn user::Repository>;
        let messages = Arc::clone(&db) as Arc<dyn message::Repository>;
        let refresh_tokens = Arc::clone(&db) as Arc<dyn refresh_token::Repository>;
        let relations = Arc::clone(&db) as Arc<dyn relation::Repository>;

        let logs = Arc::new(FileWriter::new(log_path.to_string())) as Arc<dyn log::Repository>;

        Arc::new(Self {
            chats,
            servers,
            channel_parents,
            channels,
            users,
            messages,
            refresh_tokens,
            relations,
            logs,
        })
    }
}
