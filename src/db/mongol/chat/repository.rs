use axum::async_trait;
use bson::Document;
use futures_util::StreamExt;
use mongodb::bson::{doc, from_document};

use crate::{db::mongol, model::{chat::{self, Chat}, error }};
use crate::db::mongol::{MongolChat, MongolChatWrapper, MongolDB};
use crate::{map_mongo_key_to_string, map_mongo_collection_keys_to_string};
use super::helper;

#[async_trait]
impl chat::Repository for MongolDB
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

    async fn update_chat(&self, chat: Chat) -> Result<(), error::Server>
    {
        let filter: Document;
        let update = match chat
        {
            Chat::Private(_) => 
            {
                return Ok(());
            },
            Chat::Group(group) => 
            {
                let id = mongol::helper::convert_domain_id_to_mongol(&group.id)?;
                filter = doc! 
                {
                    "_id": id
                };

                let user_ids: Vec<&str> = group.users.iter().map(|user| &*user.id).collect();

                doc!
                {
                    "chat.Group.name": group.name,
                    "chat.Group.user_ids": mongol::helper::convert_domain_ids_to_mongol(&user_ids)?,
                }
            },
        };

        match self.chats().update_one(filter, update).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(error::Server::FailedUpdate(err.to_string()))
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
            "_id": chat_info_id_local
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
