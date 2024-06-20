use std::time::Duration;

use axum::async_trait;
use mongodb::{bson::{doc, Uuid}, options::{ClientOptions, Compressor}, Client, Collection};

use crate::{api::chat, model::{chat::{Chat, ChatError, ChatRepository}, user::{User, UserError, UserRepository}}};
use crate::db::mongoldb::model::MongolUser;

use super::{MongolBucket, MongolChat};

#[derive(Clone, Debug)]
pub struct MongolDB
{
    users: Collection<MongolUser>,
    chats: Collection<MongolChat>,
    buckets: Collection<MongolBucket>,
}

impl MongolDB
{
    pub async fn init(connection_string: &str) 
        -> Result<Self, Box<dyn std::error::Error>>
    {
        let mut client_options = ClientOptions::parse(connection_string).await?;

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
    
        let client = Client::with_options(client_options)?;

        let db = client.database("db_mogcord");

        let users: Collection<MongolUser> = db.collection("users");
        let chats: Collection<MongolChat> = db.collection("chats");
        let buckets: Collection<MongolBucket> = db.collection("buckets");

        Ok(Self 
            { 
                users : users,
                chats: chats,
                buckets: buckets,
            }
        )
    }

    pub async fn get_user_db_object_by_id(&self, user_id: &String) -> Result<MongolUser, UserError>
    {
        let user_option = self
                                              .users
                                              .find_one(doc! { "uuid" : user_id }, None)
                                              .await
                                              .map_err(|err| UserError::UnexpectedError(Some(err.to_string())))?;

        match user_option
        {
            Some(user) => Ok(user),
            None => Err(UserError::UserNotFound)
        }
    }
}

#[async_trait]
impl UserRepository for MongolDB
{
    async fn does_user_exist_by_id(&self, user_id: &String) -> Result<bool, UserError>
    {
        let user_uuid = Uuid::parse_str(user_id)
                              .map_err(|_| UserError::UserNotFound)?;

        match self.users.find_one(doc! { "_id" : user_uuid }, None).await
        {
            Ok(option) => Ok(option.is_some()),
            Err(err) => Err(UserError::UnexpectedError(Some(err.to_string())))
        }
    }

    async fn does_user_exist_by_mail(&self, user_mail: &String) -> Result<bool, UserError>
    {
        match self.users.find_one(doc! { "mail" : user_mail }, None).await
        {
            Ok(option) => Ok(option.is_some()),
            Err(err) => Err(UserError::UnexpectedError(Some(err.to_string())))
        }
    }

    async fn create_user(&self, user: User) -> Result<User, UserError>
    {
        let db_user = MongolUser::try_from(user.clone())
                                  .map_err(|err| UserError::UnexpectedError(Some(err.to_string())))?;
        
        match self.users.insert_one(&db_user, None).await
        {
            Ok(_) => Ok(user),
            Err(err) => Err(UserError::UnexpectedError(Some(err.to_string()))),
        }
    }

    async fn get_user_by_id(&self, user_id: &String) -> Result<User, UserError>
    {
        let user_uuid = Uuid::parse_str(user_id)
                              .map_err(|_| UserError::UserNotFound)?;

        let user_option = self.users.find_one(doc! { "_id": user_uuid }, None)
                                         .await
                                         .map_err(|err| UserError::UnexpectedError(Some(err.to_string())))?;
        
        match user_option 
        {
            Some(user) => Ok(User::from(user)),
            None => Err(UserError::UserNotFound),
        }
    }
}

#[async_trait]
impl ChatRepository for MongolDB
{
    async fn create_chat(&self, chat: Chat) -> Result<Chat, ChatError>
    {
        let db_chat = MongolChat::try_from(chat.clone())
                                  .map_err(|err| ChatError::UnexpectedError(Some(err.to_string())))?;

        match self.chats.insert_one(&db_chat, None).await
        {
            Ok(_) => Ok(chat),
            Err(err) => Err(ChatError::UnexpectedError(Some(err.to_string()))),
        }
    }

    async fn get_chat_by_id(&self, chat_id: &String) -> Result<Chat, ChatError>
    {
        let pipelines = vec![
            //filter
            doc! 
            {
                "$match":
                {
                    "_id": chat_id
                }
            },
            //join with owners
            doc! 
            {
                "$lookup":
                {
                    "from": "users",
                    "localField": "owner_ids",
                    "foreignField": "_id",
                    "as": "owners"
                },
            },
            //join with members
            doc! 
            {
                "$lookup":
                {
                    "from": "users",
                    "localField": "user_ids",
                    "foreignField": "_id",
                    "as": "owners"
                },
            },
            //join with buckets
            doc! 
            {
                "$lookup":
                {
                    "from": "buckets",
                    "localField": "user_ids",
                    "foreignField": "_id",
                    "as": "owners"
                },
            }
        ];

        let document = self
                                         .chats
                                         .aggregate(pipelines, None)
                                         .await
                                         .map_err(|err| ChatError::UnexpectedError(Some(err.to_string())))?;

        match chat_option
        {
            Some(chat) => Ok(),
            None => Err(ChatError::ChatNotFound),
        }
    }
}