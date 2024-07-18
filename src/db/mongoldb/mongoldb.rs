use std::time::Duration;
use mongodb::{options::{ClientOptions, Compressor}, Client, Collection};

use crate::db::mongoldb::model::MongolUser;
use super::{MongolBucket, MongolChatWrapper, MongolMessage, MongolRefreshToken, MongolRelation};

#[derive(Clone, Debug)]
pub struct MongolDB
{
    client: Client,
    users: Collection<MongolUser>,
    chats: Collection<MongolChatWrapper>,
    buckets: Collection<MongolBucket>,
    messages: Collection<MongolMessage>,
    refreshtokens: Collection<MongolRefreshToken>,
    relations: Collection<MongolRelation>
}

impl MongolDB
{
    pub async fn init(connection_string: &str) -> Result<Self, Box<dyn std::error::Error>>
    {
        let mut client_options = ClientOptions::parse(connection_string).await?;

        client_options.app_name = Some("Mogcord".to_string());
        client_options.connect_timeout = Some(Duration::from_secs(30));
        client_options.compressors = Some(
            vec!
            [
                Compressor::Snappy,
                Compressor::Zlib {
                    level: Default::default(),
                },
                Compressor::Zstd {
                    level: Default::default(),
                },
            ]
        );
    
        let client = Client::with_options(client_options)?;

        let db = client.database("db_mogcord");
        
        let users: Collection<MongolUser> = db.collection("users");
        let chats: Collection<MongolChatWrapper> = db.collection("chats");
        let buckets: Collection<MongolBucket> = db.collection("buckets");
        let messages: Collection<MongolMessage> = db.collection("messages");
        let refreshtokens: Collection<MongolRefreshToken> = db.collection("refresh_tokens");
        let relations: Collection<MongolRelation> = db.collection("relations");

        Ok(Self 
            { 
                client: client,
                users : users,
                chats: chats,
                buckets: buckets,
                messages: messages,
                refreshtokens: refreshtokens,
                relations: relations,
            }
        )
    }
}

impl MongolDB
{
    pub fn client(&self) -> &Client 
    {
        &self.client
    }

    pub fn users(&self) -> &Collection<MongolUser>
    {
        &self.users
    }

    pub fn chats(&self) -> &Collection<MongolChatWrapper> 
    {
        &self.chats
    }

    pub fn buckets(&self) -> &Collection<MongolBucket> 
    {
        &self.buckets
    }

    pub fn messages(&self) -> &Collection<MongolMessage> 
    {
        &self.messages
    }

    pub fn refresh_tokens(&self) -> &Collection<MongolRefreshToken> 
    {
        &self.refreshtokens
    }

    pub fn relations(&self) -> &Collection<MongolRelation> 
    {
        &self.relations
    }
}