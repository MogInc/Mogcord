use axum::async_trait;
use futures_util::StreamExt;
use mongodb::bson::{doc, from_document};

use crate::{convert_mongo_key_to_string, db::mongoldb::{mongol_helper, MongolChat, MongolDB}, map_mongo_collection_keys, model::{chat::{Chat, ChatRepository}, misc::ServerError }};

#[async_trait]
impl ChatRepository for MongolDB
{
    async fn create_chat(&self, chat: Chat) -> Result<Chat, ServerError>
    {
        let db_chat = MongolChat::try_from(&chat)
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

        match self.chats().insert_one(&db_chat).await
        {
            Ok(_) => Ok(chat),
            Err(err) => Err(ServerError::FailedInsert(err.to_string())),
        }
    }

    async fn get_chat_by_id(&self, chat_id: &str) -> Result<Chat, ServerError>
    {
        let chat_id_local = mongol_helper::convert_domain_id_to_mongol(&chat_id)
            .map_err(|_| ServerError::ChatNotFound)?;

        let pipelines = vec![
            //filter
            doc! 
            {
                "$match":
                {
                    "_id": chat_id_local
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
            //join with users
            doc! 
            {
                "$lookup":
                {
                    "from": "users",
                    "localField": "user_ids",
                    "foreignField": "_id",
                    "as": "users"
                },
            },
            //rename fields
            doc!
            {
                "$addFields":
                {
                    "id": convert_mongo_key_to_string!("$_id", "uuid"),
                    "owners": map_mongo_collection_keys!("$owners", "id", "uuid"),
                    "users": map_mongo_collection_keys!("$users", "id", "uuid"),
                }
            },
            //hide fields
            doc! 
            {
                "$unset": ["_id", "owner_ids", "user_ids", "owners._id"]
            },
        ];

        let mut cursor = self
            .chats()
            .aggregate(pipelines)
            .await
            .map_err(|err| ServerError::FailedRead(err.to_string()))?;
    
        let document_option = cursor
            .next()
            .await
            .transpose()
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;
    

        match document_option
        {
            Some(document) => 
            {
                let chat = from_document(document)
                    .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

                return Ok(chat);
            },
            None => Err(ServerError::ChatNotFound), 
        }
    }

    async fn does_chat_exist(&self, chat: &Chat) -> Result<bool, ServerError>
    {
        let mongol_chat = MongolChat::try_from(chat)
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

        let pipilines = vec![
            doc! 
            {
                "$match":
                {
                    "type": mongol_chat.r#type,
                    "name": mongol_chat.name,
                    "owner_ids": mongol_chat.owner_ids,
                    "user_ids": mongol_chat.user_ids,
                }
            }
        ];

        let mut cursor = self
            .chats()
            .aggregate(pipilines)
            .await
            .map_err(|err| ServerError::FailedRead(err.to_string()))?;


        let document_option = cursor
            .next()
            .await
            .transpose()
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;
    

        match document_option
        {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}
