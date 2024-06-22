use std::{result, time::Duration};

use axum::async_trait;
use mongodb::{bson::{doc, from_document, Document, Uuid}, options::{ClientOptions, Compressor}, Client, Collection, Cursor};
use futures_util::stream::StreamExt;

use crate::{convert_mongo_key_to_string, map_mongo_collection, model::{chat::{Chat, ChatError, ChatRepository}, user::{User, UserError, UserRepository}}};
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

    pub async fn get_user_db_object_by_id(&self, user_id: &String) -> Result<MongolUser, UserError>
    {
        let user_option: Option<MongolUser> = self
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
        let user_uuid: Uuid = Uuid::parse_str(user_id)
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
        let db_user: MongolUser = MongolUser::try_from(user.clone())
            .map_err(|err| UserError::UnexpectedError(Some(err.to_string())))?;
        
        match self.users.insert_one(&db_user, None).await
        {
            Ok(_) => Ok(user),
            Err(err) => Err(UserError::UnexpectedError(Some(err.to_string()))),
        }
    }

    async fn get_user_by_id(&self, user_id: &String) -> Result<User, UserError>
    {
        let user_uuid: Uuid = Uuid::parse_str(user_id)
            .map_err(|_| UserError::UserNotFound)?;

        let user_option: Option<MongolUser> = self
            .users
            .find_one(doc! { "_id": user_uuid }, None)
            .await
            .map_err(|err| UserError::UnexpectedError(Some(err.to_string())))?;
        
        match user_option 
        {
            Some(user) => Ok(User::from(user)),
            None => Err(UserError::UserNotFound),
        }
    }

    async fn get_users_by_id(&self, user_ids: Vec<String>) -> Result<Vec<User>, UserError>
    {
        let mut user_uuids : Vec<Uuid> = Vec::new();

        for user_id in user_ids
        {
            let user_uuid: Uuid = Uuid::parse_str(user_id)
                .map_err(|_| UserError::UserNotFound)?;

            user_uuids.push(user_uuid);
        }

        let pipelines = vec![
            doc! 
            { 
                "$match": 
                { 
                    "_id": { "$in": user_uuids } 
                } 
            },
            //rename fields
            doc!
            {
                "$addFields":
                {
                    "uuid": convert_mongo_key_to_string!("$_id", "uuid"),
                }
            },
            //hide fields
            doc! 
            {
                "$unset": ["_id"]
            },
        ];

        let mut cursor = self
            .users
            .aggregate(pipelines, None)
            .await
            .map_err(|err| UserError::UnexpectedError(Some(err.to_string())))?;
        
        let mut users : Vec<User> = Vec::new();

        while let Some(result) = cursor.next().await
        {
            match result
            {
                Ok(doc) => 
                {
                    let user: User = from_document(doc)
                        .map_err(|err| UserError::UnexpectedError(Some(err.to_string())))?;
                    users.push(user);
                },
                Err(_) => (),
            }
        }
    
        Ok(users)
    }
}

#[async_trait]
impl ChatRepository for MongolDB
{
    async fn create_chat(&self, chat: Chat) -> Result<Chat, ChatError>
    {
        let db_chat: MongolChat = MongolChat::try_from(chat.clone())
            .map_err(|err| ChatError::UnexpectedError(Some(err.to_string())))?;

        match self.chats.insert_one(&db_chat, None).await
        {
            Ok(_) => Ok(chat),
            Err(err) => Err(ChatError::UnexpectedError(Some(err.to_string()))),
        }
    }

    async fn get_chat_by_id(&self, chat_id: &String) -> Result<Chat, ChatError>
    {
        let chat_uuid: Uuid = Uuid::parse_str(chat_id)
            .map_err(|_| ChatError::ChatNotFound)?;

        let pipelines = vec![
            //filter
            doc! 
            {
                "$match":
                {
                    "_id": chat_uuid
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
                    "as": "members"
                },
            },
            //join with buckets
            doc! 
            {
                "$lookup":
                {
                    "from": "buckets",
                    "localField": "bucket_ids",
                    "foreignField": "_id",
                    "as": "buckets"
                },
            },
            //rename fields
            doc!
            {
                "$addFields":
                {
                    "uuid": convert_mongo_key_to_string!("$_id", "uuid"),
                    "owners": map_mongo_collection!("$owners"),
                    "members": map_mongo_collection!("$members"),
                    "buckets": map_mongo_collection!("$buckets"),
                }
            },
            //hide fields
            doc! 
            {
                "$unset": ["_id", "owner_ids", "user_ids", "bucket_ids", "owners._id"]
            },
        ];

        let mut cursor: Cursor<Document> = self
            .chats
            .aggregate(pipelines, None)
            .await
            .map_err(|err| ChatError::UnexpectedError(Some(err.to_string())))?;
    
        let document_option: Option<Document> = cursor
            .next()
            .await
            .transpose()
            .map_err(|err| ChatError::UnexpectedError(Some(err.to_string())))?;
    

        match document_option
        {
            Some(document) => 
            {
                let chat : Chat = from_document(document)
                    .map_err(|err| ChatError::InvalidChat(Some(err.to_string()), false))?;

                return Ok(chat);
            },
            None => Err(ChatError::ChatNotFound), 
        }
    }
}
