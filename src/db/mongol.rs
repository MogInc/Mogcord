pub mod helper;
mod bucket;
mod channel_parent;
mod channel;
mod message;
mod refresh_token;
mod relation;
mod user;
mod log;

use bson::doc;
pub use bucket::*;
pub use channel_parent::*;
pub use channel::*;
pub use message::*;
pub use refresh_token::*;
pub use relation::*;
pub use user::*;
pub use log::*;

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
    relations: Collection<MongolRelation>,
    logs: Collection<MongolLog>,
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
        
        println!("MongolDB connection...");

        let users: Collection<MongolUser> = db.collection("users");
        Self::internal_add_user_indexes(&users).await?;

        let chats: Collection<MongolChat> = db.collection("chats");
        Self::internal_add_chat_indexes(&chats).await?;
        
        let servers: Collection<MongolServer> = db.collection("servers");
        Self::internal_add_server_indexes(&servers).await?;

        let channels: Collection<MongolChannel> = db.collection("channels");
        
        let buckets: Collection<MongolBucket> = db.collection("buckets");
        Self::internal_add_bucket_indexes(&buckets).await?;

        let messages: Collection<MongolMessage> = db.collection("messages");
        Self::internal_add_message_indexes(&messages).await?;

        let refreshtokens: Collection<MongolRefreshToken> = db.collection("refresh_tokens");
        Self::internal_add_refresh_token_indexes(&refreshtokens).await?;
        
        let relations: Collection<MongolRelation> = db.collection("relations");
        Self::internal_add_relation_indexes(&relations).await?;
        
        let logs: Collection<MongolLog> = db.collection("logs");
        Self::internal_add_log_indexes(&logs).await?;

        println!("Mongol indexes set...");

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
                logs
            }
        )
    }

    async fn internal_add_refresh_token_indexes(coll: &Collection<MongolRefreshToken>) -> Result<(), Error>
    {
        let device_expiration_compound = IndexModel::builder()
            .keys(doc!{ "device_id": 1, "expiration_date": -1 })
            .build();

        let owner_device_compound = IndexModel::builder()
            .keys(doc!{ "owner_id": 1, "device_id": 1 })
            .build();

        coll.create_index(device_expiration_compound).await?;
        coll.create_index(owner_device_compound).await?;

        Ok(())
    }

    async fn internal_add_message_indexes(coll: &Collection<MongolMessage>) -> Result<(), Error>
    {
        let channel_timestamp_flag_compound = IndexModel::builder()
            .keys(doc!{ "channel_id": 1, "timestamp": -1, "flag": 1 })
            .build();

        coll.create_index(channel_timestamp_flag_compound).await?;

        Ok(())
    }

    async fn internal_add_server_indexes(coll: &Collection<MongolServer>) -> Result<(), Error>
    {
        let owner_index = IndexModel::builder()
            .keys(doc!{ "owner_id": 1})
            .build();

        let users_index = IndexModel::builder()
            .keys(doc!{ "user_ids": 1})
            .build();

        let channels_index = IndexModel::builder()
            .keys(doc!{ "channel_ids": 1})
            .build();

        coll.create_index(owner_index).await?;
        coll.create_index(users_index).await?;
        coll.create_index(channels_index).await?;

        Ok(())
    }

    async fn internal_add_bucket_indexes(coll: &Collection<MongolBucket>) -> Result<(), Error>
    {
        let channel_date_compound = IndexModel::builder()
            .keys(doc!{ "channel_id": 1, "date": -1 })
            .build();

        coll.create_index(channel_date_compound).await?;

        Ok(())
    }

    async fn internal_add_relation_indexes(coll: &Collection<MongolRelation>) -> Result<(), Error>
    {
        let user_friends_compound = IndexModel::builder()
            .keys(doc!{ "user_id": 1, "friend_ids": 1 })
            .build();

        let user_incoming_compound = IndexModel::builder()
            .keys(doc!{ "user_id": 1, "pending_incoming_friend_ids": 1 })
            .build();

        let user_outgoing_compound = IndexModel::builder()
            .keys(doc!{ "user_id": 1, "pending_outgoing_friend_ids": 1 })
            .build();

        let user_blocked_compound = IndexModel::builder()
            .keys(doc!{ "user_id": 1, "blocked_ids": 1 })
            .build();

        coll.create_index(user_friends_compound).await?;
        coll.create_index(user_incoming_compound).await?;
        coll.create_index(user_outgoing_compound).await?;
        coll.create_index(user_blocked_compound).await?;

        Ok(())
    }

    async fn internal_add_chat_indexes(coll: &Collection<MongolChat>) -> Result<(), Error>
    {
        let opts_sparse = IndexOptions::builder()
            .unique(true)
            .sparse(true)
            .build();

        let private_id_index = IndexModel::builder()
            .keys(doc!{ "Private._id": 1 })
            .options(opts_sparse.clone())
            .build();

        let private_owners_index = IndexModel::builder()
            .keys(doc!{ "Private.owner_ids": 1 })
            .options(opts_sparse.clone())
            .build();

        let group_id_index = IndexModel::builder()
            .keys(doc!{ "Group._id": 1 })
            .options(opts_sparse.clone())
            .build();

        let group_owner_index = IndexModel::builder()
            .keys(doc!{ "Group.owner_id": 1 })
            .options(opts_sparse.clone())
            .build();

        let group_users_index = IndexModel::builder()
            .keys(doc!{ "Group.user_ids": 1 })
            .options(opts_sparse)
            .build();

        coll.create_index(private_id_index).await?;
        coll.create_index(private_owners_index).await?;
        coll.create_index(group_id_index).await?;
        coll.create_index(group_owner_index).await?;
        coll.create_index(group_users_index).await?;

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

    async fn internal_add_log_indexes(coll: &Collection<MongolLog>) -> Result<(), Error>
    {
        let opts = IndexOptions::builder()
            .unique(true)
            .build();

        let opts_sparse = IndexOptions::builder()
            .sparse(true)
            .build();

        let req_idex = IndexModel::builder()
            .keys(doc!{ "req_id": 1 })
            .options(opts)
            .build();

        let user_index = IndexModel::builder()
            .keys(doc!{ "user_info.user_id": 1 })
            .options(opts_sparse)
            .build();

        coll.create_index(req_idex).await?;
        coll.create_index(user_index).await?;

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

    #[must_use]
    pub fn logs(&self) -> &Collection<MongolLog> 
    {
        &self.logs
    }
}