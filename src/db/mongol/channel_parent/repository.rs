use axum::async_trait;
use bson::Document;
use futures_util::StreamExt;
use mongodb::bson::{doc, from_document};

use crate::{db::{mongol, MongolChannelVecWrapper}, map_mongo_collection_to_hashmap, model::{channel_parent::{self, chat::Chat, Server}, error }};
use crate::db::mongol::MongolDB;
use crate::{map_mongo_key_to_string, map_mongo_collection_keys_to_string};
use super::{helper, MongolChat, MongolServer};

impl channel_parent::Repository for MongolDB{}

#[async_trait]
impl channel_parent::chat::Repository for MongolDB
{
    async fn create_chat(&self, chat: Chat) -> Result<Chat, error::Server>
    {
        let db_chat = MongolChat::try_from(&chat)?;

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
                return Err(error::Server::CantUpdatePrivateChat);
            },
            Chat::Group(group) => 
            {
                let chat_id = mongol::helper::convert_domain_id_to_mongol(&group.id)?;
                filter = doc! 
                {
                    "Group._id": chat_id
                };

                let user_ids: Vec<&str> = group
                    .users
                    .keys()
                    .map(AsRef::as_ref)
                    .collect();

                doc!
                {
                    "$set":
                    {
                        "Group.name": group.name,
                        "Group.user_ids": mongol::helper::convert_domain_ids_to_mongol(&user_ids)?,
                    }
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
            "$or": 
            [
                doc!{ "Private._id": chat_id_local },
                doc!{ "Group._id": chat_id_local },
            ]
        };
     
        let projection = doc!
        {
            "_id": 0
        };

        let mongol_chat = self
            .chats()
            .find_one(filter)
            .projection(projection)
            .await
            .map_err(|err| error::Server::FailedRead(err.to_string()))?
            .ok_or(error::Server::ChatNotFound)?;

        let pipeline = match &mongol_chat
        {
            MongolChat::Private { .. } => 
            {
                let mut pipeline = vec!
                [
                    doc! 
                    {
                        "$match":
                        {
                            "Private._id": chat_id_local
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
                            "Group._id": chat_id_local
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
                println!("{document:#?}");

                let chat = from_document(document)
                    .map_err(|err| error::Server::UnexpectedError(err.to_string()))?;

                return Ok(chat);
            },
            None => Err(error::Server::ChatNotFound), 
        }
    }

    async fn does_chat_exist(&self, chat: &Chat) -> Result<bool, error::Server>
    {
        let filter = match MongolChat::try_from(chat)?
        {
            MongolChat::Private(private) => 
            {
                doc!
                {
                    "Private.owner_ids": private.owner_ids,
                }
            },
            //debating on wether to allow inf groups with same users
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

        let projection = doc!
        {
            "_id": 0
        };

        match self.chats().find_one(filter).projection(projection).await
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
        let db_server = MongolServer::try_from(&server)?;
        let db_channels = MongolChannelVecWrapper::try_from(&server)?.0;

        let mut session = self
            .client()
            .start_session()
            .await
            .map_err(|err| error::Server::TransactionError(err.to_string()))?;

        session
            .start_transaction()
            .await
            .map_err(|err| error::Server::TransactionError(err.to_string()))?;

        self
            .channels()
            .insert_many(db_channels)
            .await
            .map_err(|err| error::Server::FailedInsert(err.to_string()))?;

        match self.servers().insert_one(&db_server).session(&mut session).await
        {
            Ok(_) => Ok(server),
            Err(err) => Err(error::Server::FailedInsert(err.to_string())),
        }
    }

    async fn add_user_to_server(&self, server_id: &str, user_id: &str) -> Result<(), error::Server>
    {
        let server_id_local = helper::convert_domain_id_to_mongol(server_id)?;
        let user_id_local = helper::convert_domain_id_to_mongol(user_id)?;

        let filter = doc!
        {
            "_id": server_id_local,
        };

        let update = doc!
        {
            "$push": { "user_ids": user_id_local }
        };

        match self.servers().update_one(filter, update).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(error::Server::FailedUpdate(err.to_string())),
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

        pipeline.push(doc! { "$limit": 1 });

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
            None => Err(error::Server::ServerNotFound), 
        }
    }

    async fn get_server_by_channel_id(&self, channel_id: &str) -> Result<Server, error::Server>
    {
        let channel_id_local = helper::convert_domain_id_to_mongol(channel_id)?;

        let mut pipeline = vec!
        [
            doc! 
            {
                "$match":
                {
                    "channel_ids._id": channel_id_local
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
}


fn internal_private_chat_pipeline() -> [Document; 3]
{
    [
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
                "Private.id": map_mongo_key_to_string!("$Private._id", "uuid"),
                "Private.channel.id": map_mongo_key_to_string!("$Private.channel._id", "uuid"),
                "Private.owners": map_mongo_collection_keys_to_string!("$Private.owners", "_id", "id", "uuid"),
            }
        },
        doc!
        {
            "$unset": ["_id", "Private.owner_ids", "Private.owners._id", "Private.channel._id"]
        }
    ]
}

fn internal_group_chat_pipeline() -> [Document; 6]
{
    [
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
                "Group.id": map_mongo_key_to_string!("$Group._id", "uuid"),
                "Group.owner.id": map_mongo_key_to_string!("$Group.owner._id", "uuid"),
                "Group.channel.id": map_mongo_key_to_string!("$Group.channel._id", "uuid"),
                "Group.users": map_mongo_collection_keys_to_string!("$Group.users", "_id", "id", "uuid"),
            }
        },
        doc! 
        {
            "$addFields": 
            {
                "Group.users": map_mongo_collection_to_hashmap!("$Group.users", "id"),
            }
        },
        doc!
        {
            "$unset": ["_id", "Group._id", "Group.owner_id", "Group.owner._id", "Group.user_ids", "Group.channel._id"]
        }
    ]
}

fn internal_server_pipeline() -> [Document; 7]
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
            "$lookup":
            {
                "from": "channels",
                "localField": "channels_ids",
                "foreignField": "_id",
                "as": "channels"
            }
        },
        doc!
        {
            "$addFields":
            {
                "id": map_mongo_key_to_string!("$_id", "uuid"),
                "owner.id": map_mongo_key_to_string!("$owner._id", "uuid"),
                "users": map_mongo_collection_keys_to_string!("$users", "_id", "id", "uuid"),
                "channels": map_mongo_collection_keys_to_string!("$channels", "_id", "id", "uuid"),
            }
        },
        doc!
        {
            "$addFields":
            {
                "users": map_mongo_collection_to_hashmap!("$users", "id"),
                "channels": map_mongo_collection_to_hashmap!("$channels", "id"),
            }
        },
        doc!
        {
            "$unset": ["_id", "owner_id", "user_ids", "owner._id", "channel_ids", "channels._id"]
        }
    ]
}