use axum::async_trait;
use bson::Document;
use futures_util::StreamExt;
use mongodb::bson::{doc, from_document};

use crate::{map_mongo_key_to_string, db::mongoldb::{mongol_helper, MongolChat, MongolChatWrapper, MongolDB}, map_mongo_collection_keys_to_string, model::{chat::{Chat, ChatRepository}, misc::ServerError }};

#[async_trait]
impl ChatRepository for MongolDB
{
    async fn create_chat(&self, chat: Chat) -> Result<Chat, ServerError>
    {
        let db_chat = MongolChatWrapper::try_from(&chat)?;

        match self.chats().insert_one(&db_chat).await
        {
            Ok(_) => Ok(chat),
            Err(err) => Err(ServerError::FailedInsert(err.to_string())),
        }
    }

    async fn get_chat_by_id(&self, chat_id: &str) -> Result<Chat, ServerError>
    {
        let chat_id_local = mongol_helper::convert_domain_id_to_mongol(&chat_id)?;


        let filter = doc!
        {
            "_id": chat_id_local
        };


        let mongol_chat_wrapper = self
            .chats()
            .find_one(filter)
            .await
            .map_err(|err| ServerError::FailedRead(err.to_string()))?
            .ok_or(ServerError::ChatNotFound)?;


        //TODO: refactor this at some point 
        let pipelines = match &mongol_chat_wrapper.chat
        {
            MongolChat::Private { .. } => 
            {
                let filter = doc! 
                {
                    "$match":
                    {
                        "_id": chat_id_local
                    }
                };
                
                get_private_chat_pipeline(filter)
            },
            MongolChat::Group { .. } => 
            {
                let filter = doc! 
                {
                    "$match":
                    {
                        "_id": chat_id_local
                    }
                };
                
                get_group_chat_pipeline(filter)
            },
            MongolChat::Server { .. } => 
            {
                let filter = doc! 
                {
                    "$match":
                    {
                        "_id": chat_id_local
                    }
                };

                get_server_chat_pipeline(filter)
            },
        };
            

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


    async fn get_chat_by_chat_info_id(&self, chat_info_id: &str) -> Result<Chat, ServerError>
    {
        let chat_info_id_local = mongol_helper::convert_domain_id_to_mongol(&chat_info_id)?;


        let filter = doc!
        {
            "$or":
            [
              {"_id": chat_info_id_local},
              {"chat.Server.chat_infos._id": chat_info_id_local}
            ]
        };

        let mongol_chat_wrapper = self
            .chats()
            .find_one(filter)
            .await
            .map_err(|err| ServerError::FailedRead(err.to_string()))?
            .ok_or(ServerError::ChatNotFound)?;


        //TODO: refactor this at some point 
        let pipelines = match &mongol_chat_wrapper.chat
        {
            MongolChat::Private { .. } => 
            {
                let filter = doc! 
                {
                    "$match":
                    {
                        "_id": chat_info_id_local
                    }
                };

                get_private_chat_pipeline(filter)
            },
            MongolChat::Group { .. } => 
            {
                let filter = doc! 
                {
                    "$match":
                    {
                        "_id": chat_info_id_local
                    }
                };
                
                get_group_chat_pipeline(filter)
            },
            MongolChat::Server { .. } => 
            {
                let filter = doc! 
                {
                    "$match":
                    {
                        "chat.Server.chat_infos._id": chat_info_id_local
                    }
                };

                get_server_chat_pipeline(filter)
            },
        };
            

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
        // let mongol_chat = MongolChat::try_from(chat)?;

        // let pipilines = vec![
        //     doc! 
        //     {
        //         "$match":
        //         {
        //             "type": mongol_chat.r#type,
        //             "name": mongol_chat.name,
        //             "owner_ids": mongol_chat.owner_ids,
        //             "user_ids": mongol_chat.user_ids,
        //         }
        //     }
        // ];

        // let mut cursor = self
        //     .chats()
        //     .aggregate(pipilines)
        //     .await
        //     .map_err(|err| ServerError::FailedRead(err.to_string()))?;


        // let document_option = cursor
        //     .next()
        //     .await
        //     .transpose()
        //     .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;
    

        // match document_option
        // {
        //     Some(_) => Ok(true),
        //     None => Ok(false),
        // }

        Ok(false)
    }
}

fn get_private_chat_pipeline(filter: Document) -> Vec<Document>
{
    vec!
    [
        filter, 
        doc!
        {
            "$project":
            {
                "Private": "$chat.Private"
            }
        },
        doc! 
        {
            "$lookup": 
            {
                "from": "users",
                "localField": "Private.owner_ids",
                "foreignField": "_id",
                "as": "Private.owners"
            }
        },
        doc!
        {
            "$addFields":
            {
                "Private.id": map_mongo_key_to_string!("$_id", "uuid"),
                "Private.chat_info.id": map_mongo_key_to_string!("$Private.chat_info._id", "uuid"),
                "Private.owners": map_mongo_collection_keys_to_string!("$Private.owners", "_id", "id", "uuid"),
            }
        },
        doc!
        {
            "$unset": ["_id", "Private.owner_ids", "Private.owners._id", "Private.chat_info._id"]
        }
    ]
}

fn get_group_chat_pipeline(filter: Document) -> Vec<Document>
{
    vec!
    [
        filter,
        doc!
        {
            "$project":
            {
                "Group": "$chat.Group"
            }
        },
        doc! 
        {
            "$lookup": 
            {
                "from": "users",
                "localField": "Group.owner_id",
                "foreignField": "_id",
                "as": "Group.owner"
            }
        },
        doc!
        {
            "$unwind":
            {
                "path": "$Group.owner"
            }
        },
        doc!
        {
            "$lookup":
            {
                "from": "users",
                "localField": "Group.user_ids",
                "foreignField": "_id",
                "as": "Group.users"
            }
        },
        doc!
        {
            "$addFields":
            {
                "Group.id": map_mongo_key_to_string!("$_id", "uuid"),
                "Group.owner": map_mongo_key_to_string!("$Group.owner._id", "uuid"),
                "Group.chat_info.id": map_mongo_key_to_string!("$Group.chat_info._id", "uuid"),
                "Group.users": map_mongo_collection_keys_to_string!("$Group.users", "_id", "id", "uuid"),
            }
        },
        doc!
        {
            "$unset": ["_id", "Group.owner_id", "Group.user_ids", "Group.owner._id",  "Group.chat_info._id"]
        }
    ]
}

fn get_server_chat_pipeline(filter: Document) -> Vec<Document>
{
    vec!
    [
        doc!
        {
            "$project":
            {
                "Server": "$chat.Server"
            }
        },
        doc! 
        {
            "$lookup": 
            {
                "from": "users",
                "localField": "Server.owner_id",
                "foreignField": "_id",
                "as": "Server.owner"
            }
        },
        doc!
        {
            "$unwind":
            {
                "path": "$Server.owner"
            }
        },
        doc!
        {
            "$lookup":
            {
                "from": "users",
                "localField": "Server.user_ids",
                "foreignField": "_id",
                "as": "Server.users"
            }
        },
        doc!
        {
            "$addFields":
            {
                "Server.id": map_mongo_key_to_string!("$_id", "uuid"),
                "Server.owner": map_mongo_key_to_string!("$Server.owner._id", "uuid"),
                "Server.users": map_mongo_collection_keys_to_string!("$Server.users", "_id", "id", "uuid"),
                "Server.chat_infos": map_mongo_collection_keys_to_string!("$Server.chat_infos", "_id", "id", "uuid"),
            }
        },
        doc!
        {
            "$unset": ["_id", "Server.owner_id", "Server.user_ids", "Server.owner._id",  "Server.chat_infos._id"]
        }
    ]
}