use axum::async_trait;
use futures_util::StreamExt;
use mongodb::{bson::{doc, from_document, Document, Uuid}, Cursor};

use crate::{convert_mongo_key_to_string, db::mongoldb::{MongolChat, MongolDB}, map_mongo_collection, model::{chat::{Chat, ChatRepository}, misc::ServerError }};

#[async_trait]
impl ChatRepository for MongolDB
{
    async fn create_chat(&self, chat: Chat) -> Result<Chat, ServerError>
    {
        let db_chat: MongolChat = MongolChat::try_from(chat.clone())
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

        match self.chats().insert_one(&db_chat).await
        {
            Ok(_) => Ok(chat),
            Err(err) => Err(ServerError::FailedInsert(err.to_string())),
        }
    }

    async fn get_chat_by_id(&self, chat_id: &String) -> Result<Chat, ServerError>
    {
        let chat_uuid: Uuid = Uuid::parse_str(chat_id)
            .map_err(|_| ServerError::ChatNotFound)?;

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
                    "uuid": convert_mongo_key_to_string!("$_id", "uuid"),
                    "owners": map_mongo_collection!("$owners"),
                    "users": map_mongo_collection!("$users"),
                }
            },
            //hide fields
            doc! 
            {
                "$unset": ["_id", "owner_ids", "user_ids", "owners._id"]
            },
        ];

        let mut cursor: Cursor<Document> = self
            .chats()
            .aggregate(pipelines)
            .await
            .map_err(|err| ServerError::FailedRead(err.to_string()))?;
    
        let document_option: Option<Document> = cursor
            .next()
            .await
            .transpose()
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;
    

        match document_option
        {
            Some(document) => 
            {
                let chat : Chat = from_document(document)
                    .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

                return Ok(chat);
            },
            None => Err(ServerError::ChatNotFound), 
        }
    }

    async fn does_chat_exist(&self, chat: &Chat) -> Result<bool, ServerError>
    {
        let mongol_chat = MongolChat::try_from(chat.clone())
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


        let document_option: Option<Document> = cursor
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
