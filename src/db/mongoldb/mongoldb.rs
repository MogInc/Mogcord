use std::time::Duration;
use mongodb::{bson::doc, options::{ClientOptions, Compressor}, Client, Collection};

use crate::model::misc::ServerError;
use crate::db::mongoldb::model::MongolUser;

use super::{MongolBucket, MongolChat, MongolMessage};

#[derive(Clone, Debug)]
pub struct MongolDB
{
    users: Collection<MongolUser>,
    chats: Collection<MongolChat>,
    buckets: Collection<MongolBucket>,
    messages: Collection<MongolMessage>,
}

impl MongolDB
{
    pub async fn init(connection_string: &str) 
        -> Result<Self, Box<dyn std::error::Error>>
    {
        let mut client_options: ClientOptions = ClientOptions::parse(connection_string).await?;

        client_options.app_name = Some("Mogcord".to_string());
        client_options.connect_timeout = Some(Duration::from_secs(30));
        client_options.compressors = Some(vec![
            Compressor::Snappy,
            Compressor::Zlib {
                level: Default::default(),
            },
            Compressor::Zstd {
                level: Default::default(),
            },
        ]);
    
        let client: Client = Client::with_options(client_options)?;

        let db: mongodb::Database = client.database("db_mogcord");

        let users: Collection<MongolUser> = db.collection("users");
        let chats: Collection<MongolChat> = db.collection("chats");
        let buckets: Collection<MongolBucket> = db.collection("buckets");
        let messages: Collection<MongolMessage> = db.collection("messages");

        Ok(Self 
            { 
                users : users,
                chats: chats,
                buckets: buckets,
                messages: messages
            }
        )
    }

    pub fn users(&self) -> &Collection<MongolUser> {
        &self.users
    }

    pub fn chats(&self) -> &Collection<MongolChat> {
        &self.chats
    }

    pub fn buckets(&self) -> &Collection<MongolBucket> {
        &self.buckets
    }

    pub fn messages(&self) -> &Collection<MongolMessage> {
        &self.messages
    }

    pub async fn get_user_db_object_by_id(&self, user_id: &String) -> Result<MongolUser, ServerError>
    {
        let user_option: Option<MongolUser> = self
            .users
            .find_one(doc! { "uuid" : user_id }, None)
            .await
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

        match user_option
        {
            Some(user) => Ok(user),
            None => Err(ServerError::UserNotFound)
        }
    }
}
