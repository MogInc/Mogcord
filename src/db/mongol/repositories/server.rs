use axum::async_trait;
use bson::{doc, from_document, Document};
use futures_util::StreamExt;

use crate::db::MongolServer;
use crate::model::error;
use crate::model::server::{self, Server};
use crate::db::mongol::{helper, MongolDB};
use crate::{map_mongo_collection_keys_to_string, map_mongo_key_to_string};

#[async_trait]
impl server::Repository for MongolDB
{
    async fn create_server(&self, server: Server) -> Result<Server, error::Server>
    {
        let db_server = MongolServer::try_from(&server)?;

        match self.servers().insert_one(&db_server).await
        {
            Ok(_) => Ok(server),
            Err(err) => Err(error::Server::FailedInsert(err.to_string())),
        }
    }
    async fn get_server_by_id(&self, server_id: &str) -> Result<Server, error::Server>
    {
        let server_id_local = helper::convert_domain_id_to_mongol(server_id)?;

        let mut pipeline = vec!
        [
            doc! 
            {
                "$match":
                {
                    "_id": server_id_local
                }
            },
        ];

        pipeline.extend(internal_server_pipeline());


        let mut cursor = self
            .servers()
            .aggregate(pipeline)
            .await
            .map_err(|err| error::Server::FailedRead(err.to_string()))?;

        let document_option = cursor
            .next()
            .await
            .transpose()
            .map_err(|err| error::Server::UnexpectedError(err.to_string()))?;


        match document_option
        {
            Some(document) => 
            {
                let chat = from_document(document)
                    .map_err(|err| error::Server::UnexpectedError(err.to_string()))?;

                return Ok(chat);
            },
            None => Err(error::Server::ChatNotFound), 
        }
    }

    async fn get_server_by_chat_info_id(&self, chat_info_id: &str) -> Result<Server, error::Server>
    {
        todo!()
    }
}

fn internal_server_pipeline() -> [Document; 5]
{
    [
        doc! 
        {
            "$lookup": 
            {
                "from": "users",
                "localField": "owner_id",
                "foreignField": "_id",
                "as": "owner"
            }
        },
        doc!
        {
            "$unwind":
            {
                "path": "$owner"
            }
        },
        doc!
        {
            "$lookup":
            {
                "from": "users",
                "localField": "user_ids",
                "foreignField": "_id",
                "as": "users"
            }
        },
        doc!
        {
            "$addFields":
            {
                "id": map_mongo_key_to_string!("$_id", "uuid"),
                "owner.id": map_mongo_key_to_string!("$owner._id", "uuid"),
                "users": map_mongo_collection_keys_to_string!("$users", "_id", "id", "uuid"),
                "chat_infos": map_mongo_collection_keys_to_string!("$chat_infos", "_id", "id", "uuid"),
            }
        },
        doc!
        {
            "$unset": ["_id", "owner_id", "user_ids", "owner._id",  "chat_infos._id"]
        }
    ]
}