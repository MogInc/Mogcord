use axum::async_trait;
use bson::Document;
use futures_util::StreamExt;
use mongodb::bson::{doc, from_document};

use crate::model::{chat::{Chat, ChatRepository}, error };
use crate::db::mongol::{helper, MongolChat, MongolChatWrapper, MongolDB};
use crate::{map_mongo_key_to_string, map_mongo_collection_keys_to_string};

#[async_trait]
impl ChatRepository for MongolDB
{
    async fn create_chat(&self, chat: Chat) -> Result<Chat, error::Server>
    {
        let db_chat = MongolChatWrapper::try_from(&chat)?;

        match self.chats().insert_one(&db_chat).await
        {
            Ok(_) => Ok(chat),
            Err(err) => Err(error::Server::FailedInsert(err.to_string())),
        }
    }

    async fn get_chat_by_id(&self, chat_id: &str) -> Result<Chat, error::Server>
    {
        let chat_id_local = helper::convert_domain_id_to_mongol(chat_id)?;

        //TODO: refactor this at some point 
        //currently doing 2 db calls
        //see if it can be done in 1
        let filter = doc!
        {
            "_id": chat_id_local
        };


        let mongol_chat_wrapper = self
            .chats()
            .find_one(filter)
            .await
            .map_err(|err| error::Server::FailedRead(err.to_string()))?
            .ok_or(error::Server::ChatNotFound)?;

        let pipeline = match &mongol_chat_wrapper.chat
        {
            MongolChat::Private { .. } => 
            {
                let mut pipeline = vec!
                [
                    doc! 
                    {
                        "$match":
                        {
                            "_id": chat_id_local
                        }
                    },
                ];

                pipeline.extend(internal_private_chat_pipeline());

                pipeline
            },
            MongolChat::Group { .. } => 
            {
                let mut pipeline = vec!
                [
                    doc! 
                    {
                        "$match":
                        {
                            "_id": chat_id_local
                        }
                    },
                ];

                pipeline.extend(internal_group_chat_pipeline());

                pipeline
            },
            MongolChat::Server { .. } => 
            {
                let mut pipeline = vec!
                [
                    doc! 
                    {
                        "$match":
                        {
                            "_id": chat_id_local
                        }
                    },
                ];

                pipeline.extend(internal_server_chat_pipeline());

                pipeline
            },
        };
            

        let mut cursor = self
            .chats()
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


    async fn get_chat_by_chat_info_id(&self, chat_info_id: &str) -> Result<Chat, error::Server>
    {
        let chat_info_id_local = helper::convert_domain_id_to_mongol(chat_info_id)?;


        //TODO: refactor this at some point 
        //currently doing 2 db calls
        //see if it can be done in 1
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
            .map_err(|err| error::Server::FailedRead(err.to_string()))?
            .ok_or(error::Server::ChatNotFound)?;


        let pipeline = match &mongol_chat_wrapper.chat
        {
            MongolChat::Private { .. } => 
            {
                let mut pipeline = vec!
                [
                    doc! 
                    {
                        "$match":
                        {
                            "_id": chat_info_id_local
                        }
                    },
                ];

                pipeline.extend(internal_private_chat_pipeline());

                pipeline
            },
            MongolChat::Group { .. } => 
            {
                let mut pipeline = vec!
                [
                    doc! 
                    {
                        "$match":
                        {
                            "_id": chat_info_id_local
                        }
                    },
                ];

                pipeline.extend(internal_group_chat_pipeline());

                pipeline
            },
            MongolChat::Server { .. } => 
            {

                let mut pipeline = vec!
                [
                    doc! 
                    {
                        "$match":
                        {
                            "chat.Server.chat_infos._id": chat_info_id_local
                        }
                    },
                ];

                pipeline.extend(internal_server_chat_pipeline());

                pipeline
            },
        };
            

        let mut cursor = self
            .chats()
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


    async fn does_chat_exist(&self, chat: &Chat) -> Result<bool, error::Server>
    {
        let mongol_chat_wrapper = MongolChatWrapper::try_from(chat)?;

        let filter = match &mongol_chat_wrapper.chat
        {
            MongolChat::Private { owner_ids, .. } => 
            {
                doc!
                {
                    "chat.Private.owner_ids": owner_ids,
                }
            },
            MongolChat::Group { name, owner_id, user_ids, .. } => 
            {
                doc!
                {
                    "chat.Group.name": name,
                    "chat.Group.owner_id": owner_id,
                    "chat.Group.user_ids": user_ids,
                }
            },
            MongolChat::Server { .. } => 
            {
                return Ok(false);
            },
        };


        match self.chats().find_one(filter).await
        {
            Ok(chat_option) => Ok(chat_option.is_some()),
            Err(err) => Err(error::Server::FailedRead(err.to_string())),
        }
    }
}

fn internal_private_chat_pipeline() -> [Document; 4]
{
    [
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

fn internal_group_chat_pipeline() -> [Document; 6]
{
    [
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
                "Group.owner.id": map_mongo_key_to_string!("$Group.owner._id", "uuid"),
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

fn internal_server_chat_pipeline() -> [Document; 6]
{
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
                "Server.owner.id": map_mongo_key_to_string!("$Server.owner._id", "uuid"),
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