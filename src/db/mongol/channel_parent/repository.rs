use axum::async_trait;
use bson::Document;
use futures_util::StreamExt;
use mongodb::bson::{doc, from_document};

use crate::{db::{mongol, MongolChannel}, model::{channel_parent::{self, chat::Chat, Server}, error }};
use crate::db::mongol::MongolDB;
use crate::{map_mongo_key_to_string, map_mongo_collection_keys_to_string};
use super::{helper, MongolChannelParent, MongolChat};

impl channel_parent::Repository for MongolDB{}

#[async_trait]
impl channel_parent::chat::Repository for MongolDB
{
    async fn create_chat(&self, chat: Chat) -> Result<Chat, error::Server>
    {
        let db_chat = MongolChannelParent::try_from(&chat)?;

        match self.channel_parents().insert_one(&db_chat).await
        {
            Ok(_) => Ok(chat),
            Err(err) => Err(error::Server::FailedInsert(err.to_string())),
        }
    }

    async fn update_chat(&self, chat: Chat) -> Result<(), error::Server>
    {
        todo!()
    }

    async fn get_chat_by_id(&self, chat_id: &str) -> Result<Chat, error::Server>
    {
        todo!()
    }

    async fn does_chat_exist(&self, chat: &Chat) -> Result<bool, error::Server>
    {
        return Ok(false);

        let filter = match MongolChat::try_from(chat)?
        {
            MongolChat::Private(private) => 
            {
                doc!
                {
                    "Private.owner_ids": private.owner_ids,
                }
            },
            MongolChat::Group(group) => 
            {
                doc!
                {
                    "Group.name": group.name,
                    "Group.owner_id": group.owner_id,
                    "Group.user_ids": group.user_ids,
                }
            },
        };

        match self.channel_parents().find_one(filter).await
        {
            Ok(chat_option) => Ok(chat_option.is_some()),
            Err(err) => Err(error::Server::FailedRead(err.to_string())),
        }
    }
}

#[async_trait]
impl channel_parent::server::Repository for MongolDB
{
    async fn create_server(&self, server: Server) -> Result<Server, error::Server>
    {
        todo!()
    }

    async fn add_user_to_server(&self, server_id: &str, user_id: &str) -> Result<(), error::Server>
    {
        todo!()
    }

    async fn get_server_by_id(&self, server_id: &str) -> Result<Server, error::Server>
    {
        todo!()
    }

    async fn get_server_by_channel_id(&self, channel_id: &str) -> Result<Server, error::Server>
    {
        todo!()
    }
}