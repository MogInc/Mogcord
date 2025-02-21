use axum::async_trait;
use bson::Document;
use futures_util::StreamExt;
use mongodb::bson::{
    doc,
    from_document,
};

use super::{
    helper,
    MongolChat,
    MongolServer,
};
use crate::db::mongol::{
    self,
    MongolDB,
};
use crate::db::{
    MongolChannel,
    MongolChannelVecWrapper,
};
use crate::model::channel_parent::chat::Chat;
use crate::model::channel_parent::{
    self,
    ChannelParent,
    Server,
};
use crate::model::error;
use crate::{
    bubble,
    map_mongo_collection_keys_to_string,
    map_mongo_collection_to_hashmap,
    map_mongo_key_to_string,
    server_error,
    transaction_error,
};

#[async_trait]
impl channel_parent::Repository for MongolDB
{
    async fn get_channel_parent<'input, 'err>(
        &'input self,
        channel_id: &'input str,
    ) -> error::Result<'err, ChannelParent>
    {
        let channel_id_local =
            bubble!(helper::convert_domain_id_to_mongol(channel_id))?;

        let mongol_channel = self
            .channels()
            .find_one(doc! { "_id": channel_id_local })
            .await
            .map_err(|err| {
                server_error!(
                    error::Kind::Fetch,
                    error::OnType::ChannelParent
                )
                .add_debug_info("error", err.to_string())
            })?
            .ok_or(
                server_error!(
                    error::Kind::NotFound,
                    error::OnType::Channel
                )
                .add_debug_info(
                    "channel id",
                    channel_id.to_string(),
                ),
            )?;

        let mut cursor = match mongol_channel.parent_type
        {
            mongol::ParentType::ChatPrivate =>
            {
                let mut pipeline = Vec::new();

                pipeline.push(doc! {
                   "$match":
                   {
                        "Private._id": channel_id_local
                   }
                });

                pipeline.extend(internal_private_chat_pipeline());

                self.chats().aggregate(pipeline).await.map_err(|err| {
                    server_error!(
                        error::Kind::Fetch,
                        error::OnType::ChatPrivate
                    )
                    .add_debug_info("error", err.to_string())
                })?
            },
            mongol::ParentType::ChatGroup =>
            {
                let mut pipeline = Vec::new();

                pipeline.push(doc! {
                     "$match":
                     {
                        "Group._id": channel_id_local
                     }
                });

                pipeline.extend(internal_group_chat_pipeline());

                self.chats().aggregate(pipeline).await.map_err(|err| {
                    server_error!(
                        error::Kind::Fetch,
                        error::OnType::ChatGroup
                    )
                    .add_debug_info("error", err.to_string())
                })?
            },
            mongol::ParentType::Server =>
            {
                let mut pipeline = Vec::new();

                pipeline.push(doc! {
                    "$match":
                    {
                        "channel_ids": channel_id_local
                    }
                });

                pipeline.extend(internal_server_pipeline());

                pipeline.push(doc! {
                    "$replaceWith":
                    {
                        "Server": "$$ROOT"
                    }
                });

                self.servers().aggregate(pipeline).await.map_err(|err| {
                    server_error!(
                        error::Kind::Fetch,
                        error::OnType::Server
                    )
                    .add_debug_info("error", err.to_string())
                })?
            },
        };

        let document_option =
            cursor.next().await.transpose().map_err(|err| {
                server_error!(
                    error::Kind::Unexpected,
                    error::OnType::ChannelParent
                )
                .add_debug_info("error", err.to_string())
            })?;

        match document_option
        {
            Some(document) =>
            {
                let channel_parent =
                    from_document(document).map_err(|err| {
                        server_error!(
                            error::Kind::Parse,
                            error::OnType::ChannelParent
                        )
                        .add_debug_info("error", err.to_string())
                    })?;

                Ok(channel_parent)
            },
            None => Err(server_error!(
                error::Kind::Unexpected,
                error::OnType::ChannelParent
            )),
        }
    }
}

#[async_trait]
impl channel_parent::chat::Repository for MongolDB
{
    async fn create_chat<'input, 'err>(
        &'input self,
        chat: Chat,
    ) -> error::Result<'err, Chat>
    {
        let db_chat = bubble!(MongolChat::try_from(&chat))?;
        let db_channel = bubble!(MongolChannel::try_from(&chat))?;

        let mut session = self
            .client()
            .start_session()
            .await
            .map_err(|err| transaction_error!(err))?;

        session
            .start_transaction()
            .await
            .map_err(|err| transaction_error!(err))?;

        self.channels()
            .insert_one(db_channel)
            .session(&mut session)
            .await
            .map_err(|err| {
                server_error!(
                    error::Kind::Insert,
                    error::OnType::Chat
                )
                .add_debug_info("error", err.to_string())
            })?;

        match self
            .chats()
            .insert_one(&db_chat)
            .session(&mut session)
            .await
        {
            Ok(_) =>
            {
                session
                    .commit_transaction()
                    .await
                    .map_err(|err| transaction_error!(err))?;

                Ok(chat)
            },
            Err(err) =>
            {
                session
                    .abort_transaction()
                    .await
                    .map_err(|err| transaction_error!(err))?;

                Err(server_error!(
                    error::Kind::Insert,
                    error::OnType::Chat
                )
                .add_debug_info("error", err.to_string()))
            },
        }
    }

    async fn update_chat<'input, 'err>(
        &'input self,
        chat: Chat,
    ) -> error::Result<'err, ()>
    {
        let filter: Document;
        let update = match chat
        {
            Chat::Private(_) =>
            {
                return Err(server_error!(
                    error::Kind::Update,
                    error::OnType::ChatPrivate
                )
                .add_client(error::Client::PRIVATE_CHAT_TRY_EDIT));
            },
            Chat::Group(group) =>
            {
                let chat_id = bubble!(
                    mongol::helper::convert_domain_id_to_mongol(&group.id)
                )?;
                filter = doc! {
                    "Group._id": chat_id
                };

                let user_ids: Vec<&str> =
                    group.users.keys().map(AsRef::as_ref).collect();

                doc! {
                    "$set":
                    {
                        "Group.name": group.name,
                        "Group.user_ids": bubble!(mongol::helper::convert_domain_ids_to_mongol(&user_ids))?,
                    }
                }
            },
        };

        match self.chats().update_one(filter, update).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(server_error!(
                error::Kind::Update,
                error::OnType::Chat
            )
            .add_debug_info("error", err.to_string())),
        }
    }

    async fn get_chat_by_id<'input, 'err>(
        &'input self,
        chat_id: &'input str,
    ) -> error::Result<'err, Chat>
    {
        let chat_id_local =
            bubble!(helper::convert_domain_id_to_mongol(chat_id))?;

        //TODO: refactor this at some point
        //currently doing 2 db calls
        //see if it can be done in 1
        let filter = doc! {
            "$or":
            [
                doc!{ "Private._id": chat_id_local },
                doc!{ "Group._id": chat_id_local },
            ]
        };

        let projection = doc! {
            "_id": 0
        };

        let mongol_chat = self
            .chats()
            .find_one(filter)
            .projection(projection)
            .await
            .map_err(|err| {
                server_error!(
                    error::Kind::Fetch,
                    error::OnType::Chat
                )
                .add_debug_info("error", err.to_string())
            })?
            .ok_or(
                server_error!(
                    error::Kind::NotFound,
                    error::OnType::Chat
                )
                .add_debug_info("chat id", chat_id.to_string()),
            )?;

        let pipeline = match &mongol_chat
        {
            MongolChat::Private {
                ..
            } =>
            {
                let mut pipeline = vec![doc! {
                    "$match":
                    {
                        "Private._id": chat_id_local
                    }
                }];

                pipeline.extend(internal_private_chat_pipeline());

                pipeline
            },
            MongolChat::Group {
                ..
            } =>
            {
                let mut pipeline = vec![doc! {
                    "$match":
                    {
                        "Group._id": chat_id_local
                    }
                }];

                pipeline.extend(internal_group_chat_pipeline());

                pipeline
            },
        };

        let mut cursor =
            self.chats().aggregate(pipeline).await.map_err(|err| {
                server_error!(
                    error::Kind::Fetch,
                    error::OnType::Chat
                )
                .add_debug_info("error", err.to_string())
            })?;

        let document_option =
            cursor.next().await.transpose().map_err(|err| {
                server_error!(
                    error::Kind::Unexpected,
                    error::OnType::Chat
                )
                .add_debug_info("error", err.to_string())
            })?;

        match document_option
        {
            Some(document) =>
            {
                let chat = from_document(document).map_err(|err| {
                    server_error!(
                        error::Kind::Parse,
                        error::OnType::Chat
                    )
                    .add_debug_info("error", err.to_string())
                })?;

                Ok(chat)
            },
            None => Err(server_error!(
                error::Kind::NotFound,
                error::OnType::Chat
            )
            .add_debug_info("chat id", chat_id.to_string())),
        }
    }

    async fn does_chat_exist<'input, 'err>(
        &'input self,
        chat: &'input Chat,
    ) -> error::Result<'err, bool>
    {
        let filter = match bubble!(MongolChat::try_from(chat))?
        {
            MongolChat::Private(private) =>
            {
                doc! {
                    "Private.owner_ids": private.owner_ids,
                }
            },
            //debating on wether to allow inf groups with same users
            MongolChat::Group(group) =>
            {
                doc! {
                    "Group.name": group.name,
                    "Group.owner_id": group.owner_id,
                    "Group.user_ids": group.user_ids,
                }
            },
        };

        let projection = doc! {
            "_id": 0
        };

        match self.chats().find_one(filter).projection(projection).await
        {
            Ok(chat_option) => Ok(chat_option.is_some()),
            Err(err) => Err(server_error!(
                error::Kind::Fetch,
                error::OnType::Chat
            )
            .add_debug_info("error", err.to_string())),
        }
    }
}

#[async_trait]
impl channel_parent::server::Repository for MongolDB
{
    async fn create_server<'input, 'err>(
        &'input self,
        server: Server,
    ) -> error::Result<'err, Server>
    {
        let db_server = bubble!(MongolServer::try_from(
            &server
        ))?;
        let db_channels =
            bubble!(MongolChannelVecWrapper::try_from(&server))?.0;

        let mut session = self
            .client()
            .start_session()
            .await
            .map_err(|err| transaction_error!(err))?;

        session
            .start_transaction()
            .await
            .map_err(|err| transaction_error!(err))?;

        self.channels()
            .insert_many(db_channels)
            .session(&mut session)
            .await
            .map_err(|err| {
                server_error!(
                    error::Kind::Insert,
                    error::OnType::Channel
                )
                .add_debug_info("error", err.to_string())
            })?;

        match self
            .servers()
            .insert_one(&db_server)
            .session(&mut session)
            .await
        {
            Ok(_) =>
            {
                session
                    .commit_transaction()
                    .await
                    .map_err(|err| transaction_error!(err))?;

                Ok(server)
            },
            Err(err) =>
            {
                session
                    .abort_transaction()
                    .await
                    .map_err(|err| transaction_error!(err))?;

                Err(server_error!(
                    error::Kind::Insert,
                    error::OnType::Server
                )
                .add_debug_info("error", err.to_string()))
            },
        }
    }

    async fn add_user_to_server<'input, 'err>(
        &'input self,
        server_id: &'input str,
        user_id: &'input str,
    ) -> error::Result<'err, ()>
    {
        let server_id_local =
            bubble!(helper::convert_domain_id_to_mongol(server_id))?;
        let user_id_local =
            bubble!(helper::convert_domain_id_to_mongol(user_id))?;

        let filter = doc! {
            "_id": server_id_local,
        };

        let update = doc! {
            "$push": { "user_ids": user_id_local }
        };

        match self.servers().update_one(filter, update).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(server_error!(
                error::Kind::CantGainUsers,
                error::OnType::Server
            )
            .add_debug_info("error", err.to_string())
            .add_debug_info(
                "server id",
                server_id.to_string(),
            )
            .add_debug_info(
                "user to add",
                user_id.to_string(),
            )),
        }
    }

    async fn get_server_by_id<'input, 'err>(
        &'input self,
        server_id: &'input str,
    ) -> error::Result<'err, Server>
    {
        let server_id_local =
            bubble!(helper::convert_domain_id_to_mongol(server_id))?;

        let mut pipeline = vec![doc! {
            "$match":
            {
                "_id": server_id_local
            }
        }];

        pipeline.extend(internal_server_pipeline());

        let mut cursor =
            self.servers().aggregate(pipeline).await.map_err(|err| {
                server_error!(
                    error::Kind::Fetch,
                    error::OnType::Server
                )
                .add_debug_info("error", err.to_string())
            })?;

        let document_option =
            cursor.next().await.transpose().map_err(|err| {
                server_error!(
                    error::Kind::Unexpected,
                    error::OnType::Server
                )
                .add_debug_info("error", err.to_string())
            })?;

        match document_option
        {
            Some(document) =>
            {
                let server = from_document(document).map_err(|err| {
                    server_error!(
                        error::Kind::Unexpected,
                        error::OnType::Server
                    )
                    .add_debug_info("error", err.to_string())
                })?;

                Ok(server)
            },
            None => Err(server_error!(
                error::Kind::NotFound,
                error::OnType::Server
            )
            .add_client(error::Client::SERVER_NOT_FOUND)),
        }
    }

    async fn get_server_by_channel_id<'input, 'err>(
        &'input self,
        channel_id: &'input str,
    ) -> error::Result<'err, Server>
    {
        let channel_id_local =
            bubble!(helper::convert_domain_id_to_mongol(channel_id))?;

        let mut pipeline = vec![doc! {
            "$match":
            {
                "channel_ids._id": channel_id_local
            }
        }];

        pipeline.extend(internal_server_pipeline());

        let mut cursor =
            self.servers().aggregate(pipeline).await.map_err(|err| {
                server_error!(
                    error::Kind::Fetch,
                    error::OnType::Server
                )
                .add_debug_info("error", err.to_string())
            })?;

        let document_option =
            cursor.next().await.transpose().map_err(|err| {
                server_error!(
                    error::Kind::Unexpected,
                    error::OnType::Server
                )
                .add_debug_info("error", err.to_string())
            })?;

        match document_option
        {
            Some(document) =>
            {
                let server = from_document(document).map_err(|err| {
                    server_error!(
                        error::Kind::Parse,
                        error::OnType::Server
                    )
                    .add_debug_info("error", err.to_string())
                })?;

                return Ok(server);
            },
            None => Err(server_error!(
                error::Kind::NotFound,
                error::OnType::Server
            )),
        }
    }
}

fn internal_private_chat_pipeline() -> [Document; 5]
{
    [
        doc! {
            "$lookup":
            {
                "from": "users",
                "localField": "Private.owner_ids",
                "foreignField": "_id",
                "as": "Private.owners"
            }
        },
        doc! {
            "$lookup":
            {
                "from": "channels",
                "localField": "Private.channel_id",
                "foreignField": "_id",
                "as": "Private.channel"
            }
        },
        doc! {
            "$unwind":
            {
                "path": "$Private.channel"
            }
        },
        doc! {
            "$addFields":
            {
                "Private.id": map_mongo_key_to_string!("$Private._id", "uuid"),
                "Private.channel.id": map_mongo_key_to_string!("$Private.channel._id", "uuid"),
                "Private.owners": map_mongo_collection_keys_to_string!("$Private.owners", "_id", "id", "uuid"),
            }
        },
        doc! {
            "$unset": ["_id", "Private.owner_ids", "Private.owners._id", "Private.channel._id"]
        },
    ]
}

fn internal_group_chat_pipeline() -> [Document; 8]
{
    [
        doc! {
            "$lookup":
            {
                "from": "users",
                "localField": "Group.owner_id",
                "foreignField": "_id",
                "as": "Group.owner"
            }
        },
        doc! {
            "$unwind":
            {
                "path": "$Group.owner"
            }
        },
        doc! {
            "$lookup":
            {
                "from": "channels",
                "localField": "Group.channel_id",
                "foreignField": "_id",
                "as": "Group.channel"
            }
        },
        doc! {
            "$unwind":
            {
                "path": "$Group.channel"
            }
        },
        doc! {
            "$lookup":
            {
                "from": "users",
                "localField": "Group.user_ids",
                "foreignField": "_id",
                "as": "Group.users"
            }
        },
        doc! {
            "$addFields":
            {
                "Group.id": map_mongo_key_to_string!("$Group._id", "uuid"),
                "Group.owner.id": map_mongo_key_to_string!("$Group.owner._id", "uuid"),
                "Group.channel.id": map_mongo_key_to_string!("$Group.channel._id", "uuid"),
                "Group.users": map_mongo_collection_keys_to_string!("$Group.users", "_id", "id", "uuid"),
            }
        },
        doc! {
            "$addFields":
            {
                "Group.users": map_mongo_collection_to_hashmap!("$Group.users", "id"),
            }
        },
        doc! {
            "$unset": ["_id", "Group._id", "Group.owner_id", "Group.owner._id", "Group.user_ids", "Group.channel._id"]
        },
    ]
}

fn internal_server_pipeline() -> [Document; 7]
{
    [
        doc! {
            "$lookup":
            {
                "from": "users",
                "localField": "owner_id",
                "foreignField": "_id",
                "as": "owner"
            }
        },
        doc! {
            "$unwind":
            {
                "path": "$owner"
            }
        },
        doc! {
            "$lookup":
            {
                "from": "users",
                "localField": "user_ids",
                "foreignField": "_id",
                "as": "users"
            }
        },
        doc! {
            "$lookup":
            {
                "from": "channels",
                "localField": "channel_ids",
                "foreignField": "_id",
                "as": "channels"
            }
        },
        doc! {
            "$addFields":
            {
                "id": map_mongo_key_to_string!("$_id", "uuid"),
                "owner.id": map_mongo_key_to_string!("$owner._id", "uuid"),
                "users": map_mongo_collection_keys_to_string!("$users", "_id", "id", "uuid"),
                "channels": map_mongo_collection_keys_to_string!("$channels", "_id", "id", "uuid"),
            }
        },
        doc! {
            "$addFields":
            {
                "users": map_mongo_collection_to_hashmap!("$users", "id"),
                "channels": map_mongo_collection_to_hashmap!("$channels", "id"),
            }
        },
        doc! {
            "$unset": ["_id", "owner_id", "user_ids", "owner._id", "channel_ids", "channels._id"]
        },
    ]
}
