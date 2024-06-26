use axum::async_trait;
use futures_util::StreamExt;
use mongodb::bson::{doc, from_document, Uuid};

use crate::{convert_mongo_key_to_string, db::mongoldb::{MongolDB, MongolUser}, model::{misc::{Pagination, ServerError}, user::{User, UserRepository}}};

#[async_trait]
impl UserRepository for MongolDB
{
    async fn does_user_exist_by_id(&self, user_id: &String) -> Result<bool, ServerError>
    {
        let user_uuid: Uuid = Uuid::parse_str(user_id)
            .map_err(|_| ServerError::UserNotFound)?;

        match self.users().find_one(doc! { "_id" : user_uuid }).await
        {
            Ok(option) => Ok(option.is_some()),
            Err(err) => Err(ServerError::UnexpectedError(err.to_string()))
        }
    }

    async fn does_user_exist_by_mail(&self, user_mail: &String) -> Result<bool, ServerError>
    {
        match self.users().find_one(doc! { "mail" : user_mail }).await
        {
            Ok(option) => Ok(option.is_some()),
            Err(err) => Err(ServerError::UnexpectedError(err.to_string()))
        }
    }

    async fn create_user(&self, user: User) -> Result<User, ServerError>
    {
        let db_user: MongolUser = MongolUser::try_from(user.clone())
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;
        
        match self.users().insert_one(&db_user).await
        {
            Ok(_) => Ok(user),
            Err(err) => Err(ServerError::UnexpectedError(err.to_string())),
        }
    }

    async fn get_user_by_id(&self, user_id: &String) -> Result<User, ServerError>
    {
        let user_uuid: Uuid = Uuid::parse_str(user_id)
            .map_err(|_| ServerError::UserNotFound)?;

        let user_option: Option<MongolUser> = self
            .users()
            .find_one(doc! { "_id": user_uuid })
            .await
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;
        
        match user_option 
        {
            Some(user) => Ok(User::from(user)),
            None => Err(ServerError::UserNotFound),
        }
    }

    async fn get_users_by_ids(&self, user_ids: Vec<String>) -> Result<Vec<User>, ServerError>
    {
        let mut user_uuids : Vec<Uuid> = Vec::new();

        for user_id in user_ids
        {
            let user_uuid: Uuid = Uuid::parse_str(user_id)
                .map_err(|_| ServerError::UserNotFound)?;

            user_uuids.push(user_uuid);
        }

        let pipelines = vec![
            doc! 
            { 
                "$match": 
                { 
                    "_id": { "$in": user_uuids } 
                } 
            },
            //rename fields
            doc!
            {
                "$addFields":
                {
                    "uuid": convert_mongo_key_to_string!("$_id", "uuid"),
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
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;
        
        let mut users : Vec<User> = Vec::new();

        while let Some(result) = cursor.next().await
        {
            match result
            {
                Ok(doc) => 
                {
                    let user: User = from_document(doc)
                        .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;
                    users.push(user);
                },
                Err(_) => (),
            }
        }
    
        Ok(users)
    }

    async fn get_users(&self, pagination: Pagination) -> Result<Vec<User>, ServerError>
    {
        let pipelines = vec![
            //rename fields
            doc!
            {
                "$addFields":
                {
                    "uuid": convert_mongo_key_to_string!("$_id", "uuid"),
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
                "$skip": pagination.get_skip_size() as i32
            },
            //limit output
            doc! 
            {
                "$limit": pagination.page_size as i32
            },
        ];

        let mut cursor = self
            .users()
            .aggregate(pipelines)
            .await
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;
        
        let mut users : Vec<User> = Vec::new();

        while let Some(result) = cursor.next().await
        {
            match result
            {
                Ok(doc) => 
                {
                    let user: User = from_document(doc)
                        .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;
                    users.push(user);
                },
                Err(_) => (),
            }
        }
    
        Ok(users)
    }
}