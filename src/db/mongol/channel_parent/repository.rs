use axum::async_trait;
use bson::Document;
use futures_util::StreamExt;
use mongodb::bson::{doc, from_document};

use crate::{db::mongol, model::{channel_parent::{self, ChannelParent}, error }};
use crate::db::mongol::{MongolChat, MongolChatWrapper, MongolDB};
use crate::{map_mongo_key_to_string, map_mongo_collection_keys_to_string};
use super::helper;

#[async_trait]
impl channel_parent::Repository for MongolDB
{

}