mod model;
mod repositories;
mod mongol_helper;

pub use model::*;
pub use mongol_helper::*;

use std::time::Duration;
use mongodb::{options::{ClientOptions, Compressor}, Client, Collection};


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
                    level: Option::default(),
                },
                Compressor::Zstd {
                    level: Option::default(),
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
                client,
                users,
                chats,
                buckets,
                messages,
                refreshtokens,
                relations,
            }
        )
    }
}

impl MongolDB
{
    #[must_use]
    pub fn client(&self) -> &Client 
    {
        &self.client
    }
    
    #[must_use]
    pub fn users(&self) -> &Collection<MongolUser>
    {
        &self.users
    }
    
    #[must_use]
    pub fn chats(&self) -> &Collection<MongolChatWrapper> 
    {
        &self.chats
    }
    
    #[must_use]
    pub fn buckets(&self) -> &Collection<MongolBucket> 
    {
        &self.buckets
    }
    
    #[must_use]
    pub fn messages(&self) -> &Collection<MongolMessage> 
    {
        &self.messages
    }
    
    #[must_use]
    pub fn refresh_tokens(&self) -> &Collection<MongolRefreshToken> 
    {
        &self.refreshtokens
    }
    
    #[must_use]
    pub fn relations(&self) -> &Collection<MongolRelation> 
    {
        &self.relations
    }
}