use axum::async_trait;
use bson::Document;
use futures_util::StreamExt;
use mongodb::bson::{doc, from_document, Uuid};

use crate::model::{error, Pagination, user::{self, User, UserFlag}};
use crate::db::mongol::{helper, MongolDB, MongolUser, MongolUserVec};
use crate::map_mongo_key_to_string;

#[async_trait]
impl user::Repository for MongolDB
{
    async fn does_user_exist_by_id(&self, user_id: &str) -> Result<bool, error::Server>
    {
        let user_id_local = helper::convert_domain_id_to_mongol(user_id)?;

        let filter = doc! { "_id" : user_id_local };

        internal_does_user_exist(self, filter).await
    }

    async fn does_user_exist_by_mail(&self, user_mail: &str) -> Result<bool, error::Server>
    {
        let filter = doc! { "mail" : user_mail };

        internal_does_user_exist(self, filter).await
    }

    async fn does_user_exist_by_username(&self, username: &str) -> Result<bool, error::Server>
    {
        let filter = doc! { "username" : username };

        internal_does_user_exist(self, filter).await
    }

    async fn create_user(&self, user: User) -> Result<User, error::Server>
    {
        let db_user = MongolUser::try_from(&user)?;
        
        match self.users().insert_one(&db_user).await
        {
            Ok(_) => Ok(user),
            Err(err) => Err(error::Server::FailedInsert(err.to_string())),
        }
    }

    async fn create_users(&self, users: Vec<User>) -> Result<(), error::Server>
    {
        let db_users = MongolUserVec::try_from(&users)?;
        
        match self.users().insert_many(&db_users.0).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(error::Server::FailedInsert(err.to_string())),
        }
    }

    async fn get_user_by_id(&self, user_id: &str) -> Result<User, error::Server>
    {
        let user_id_local = helper::convert_domain_id_to_mongol(user_id)?;

        let filter = doc! { "_id": user_id_local };

        internal_get_user(self, filter).await
    }

    async fn get_user_by_mail(&self, mail: &str) -> Result<User, error::Server>
    {
        let filter = doc! { "mail": mail };

        internal_get_user(self, filter).await
    }

    async fn get_users_by_id(&self, user_ids: Vec<String>) -> Result<Vec<User>, error::Server>
    {
        let mut user_ids_local : Vec<Uuid> = Vec::new();

        for user_id in user_ids
        {
            let user_id: Uuid = helper::convert_domain_id_to_mongol(&user_id)?;

            user_ids_local.push(user_id);
        }

        let pipelines = vec!
        [
            doc! 
            { 
                "$match": 
                { 
                    "_id": { "$in": user_ids_local },
                } 
            },
            //rename fields
            doc!
            {
                "$addFields":
                {
                    "id": map_mongo_key_to_string!("$_id", "uuid"),
                }
            },
            //hide fields
            doc! 
            {
                "$unset": ["_id"]
            },
        ];

        let mut cursor = self
            .users()
            .aggregate(pipelines)
            .await
            .map_err(|err| error::Server::FailedRead(err.to_string()))?;
        
        let mut users : Vec<User> = Vec::new();

        while let Some(result) = cursor.next().await
        {
            match result
            {
                Ok(doc) => 
                {
                    let user: User = from_document(doc)
                        .map_err(|err| error::Server::UnexpectedError(err.to_string()))?;
                    users.push(user);
                },
                Err(err) => println!("{err}"),
            }
        }
    
        Ok(users)
    }

    async fn get_users(&self, pagination: Pagination) -> Result<Vec<User>, error::Server>
    {
        let pipelines = vec!
        [
            //rename fields
            doc!
            {
                "$addFields":
                {
                    "id": map_mongo_key_to_string!("$_id", "uuid"),
                }
            },
            //hide fields
            doc! 
            {
                "$unset": ["_id"]
            },
            //skip offset
            doc! 
            {
                "$skip": i32::try_from(pagination.get_skip_size()).ok().unwrap_or(0)
            },
            //limit output
            doc! 
            {
                "$limit": i32::try_from(pagination.page_size).ok().unwrap_or(0)
            },
        ];

        let mut cursor = self
            .users()
            .aggregate(pipelines)
            .await
            .map_err(|err| error::Server::FailedRead(err.to_string()))?;
        
        let mut users : Vec<User> = Vec::new();

        while let Some(result) = cursor.next().await
        {
            match result
            {
                Ok(doc) => 
                {
                    let user: User = from_document(doc)
                        .map_err(|err| error::Server::UnexpectedError(err.to_string()))?;
                    users.push(user);
                },
                Err(err) => println!("{err}"),
            }
        }
    
        Ok(users)
    }
}

async fn internal_does_user_exist(repo: &MongolDB, filter: Document) -> Result<bool, error::Server>
{
    match repo.users().find_one(filter).await
    {
        Ok(option) => Ok(option.is_some()),
        Err(err) => Err(error::Server::FailedRead(err.to_string()))
    }
}

async fn internal_get_user(repo: &MongolDB, filter: Document) -> Result<User, error::Server>
{
    let user_option = repo
        .users()
        .find_one(filter)
        .await
        .map_err(|err| error::Server::FailedRead(err.to_string()))?;

    match user_option 
    {
        Some(user) => Ok(User::from(&user)),
        None => Err(error::Server::UserNotFound),
    }
}


fn _internal_wrap_valid_user_filter(filter: Document) -> Document
{
    doc!
    {
        "$and":
        [
            filter,
            { "flag": _internal_valid_user_filter() },
        ]
    }
}

fn _internal_valid_user_filter() -> Document
{
    doc! { "$in": [UserFlag::None, UserFlag::Admin, UserFlag::Owner] }
}