pub mod helper;
mod bucket;
mod channel_parent;
mod channel;
mod message;
mod refresh_token;
mod relation;
mod user;

use bson::doc;
pub use bucket::*;
pub use channel_parent::*;
pub use channel::*;
pub use message::*;
pub use refresh_token::*;
pub use relation::*;
pub use user::*;

use std::time::Duration;
use mongodb::{error::Error, options::{ClientOptions, Compressor, IndexOptions}, Client, Collection, IndexModel};


#[derive(Clone, Debug)]
pub struct MongolDB
{
    client: Client,
    users: Collection<MongolUser>,
    chats: Collection<MongolChat>,
    servers: Collection<MongolServer>,
    channels: Collection<MongolChannel>,
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
        let chats: Collection<MongolChat> = db.collection("chats");
        let servers: Collection<MongolServer> = db.collection("servers");
        let channels: Collection<MongolChannel> = db.collection("channels");
        let buckets: Collection<MongolBucket> = db.collection("buckets");
        let messages: Collection<MongolMessage> = db.collection("messages");
        let refreshtokens: Collection<MongolRefreshToken> = db.collection("refresh_tokens");
        let relations: Collection<MongolRelation> = db.collection("relations");

        Self::internal_add_user_indexes(&users).await?;
        Self::internal_add_chat_indexes(&chats).await?;
        Self::internal_add_refresh_token_indexes(&refreshtokens).await?;

        Ok(
            Self 
            { 
                client,
                users,
                chats,
                servers,
                channels,
                buckets,
                messages,
                refreshtokens,
                relations,
            }
        )
    }

    async fn internal_add_refresh_token_indexes(coll: &Collection<MongolRefreshToken>) -> Result<(), Error>
    {
        let device_id_index = IndexModel::builder()
            .keys(doc!{ "device_id": 1 })
            .build();

        coll.create_index(device_id_index).await?;

        Ok(())
    }

    async fn internal_add_chat_indexes(coll: &Collection<MongolChat>) -> Result<(), Error>
    {
        let opts_sparse = IndexOptions::builder()
            .unique(true)
            .sparse(true)
            .build();

        let private_chat_index = IndexModel::builder()
            .keys(doc!{ "Private._id": 1 })
            .options(opts_sparse.clone())
            .build();

        let group_chat_index = IndexModel::builder()
            .keys(doc!{ "Group._id": 1 })
            .options(opts_sparse)
            .build();

        coll.create_index(private_chat_index).await?;
        coll.create_index(group_chat_index).await?;

        Ok(())
    }

    async fn internal_add_user_indexes(coll: &Collection<MongolUser>) -> Result<(), Error>
    {
        let opts = IndexOptions::builder()
            .unique(true)
            .build();

        let username_index = IndexModel::builder()
            .keys(doc!{ "username": 1 })
            .options(opts.clone())
            .build();

        let mail_index = IndexModel::builder()
            .keys(doc!{ "mail": 1 })
            .options(opts)
            .build();

        coll.create_index(username_index).await?;
        coll.create_index(mail_index).await?;

        Ok(())
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
    pub fn chats(&self) -> &Collection<MongolChat> 
    {
        &self.chats
    }

    #[must_use]
    pub fn servers(&self) -> &Collection<MongolServer> 
    {
        &self.servers
    }

    #[must_use]
    pub fn channels(&self) -> &Collection<MongolChannel> 
    {
        &self.channels
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