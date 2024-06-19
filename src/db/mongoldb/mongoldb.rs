use std::time::Duration;

use axum::async_trait;
use mongodb::{bson::{doc, Uuid}, options::{ClientOptions, Compressor}, Client, Collection};

use crate::model::user::{User, UserError, UserRepository};
use crate::db::mongoldb::model::MongolUser;

#[derive(Clone, Debug)]
pub struct MongolDB
{
    users: Collection<MongolUser>
}

impl MongolDB
{
    pub async fn init(connection_string: &str) 
        -> Result<Self, Box<dyn std::error::Error>>
    {
        let mut client_options = ClientOptions::parse(connection_string).await?;

        client_options.app_name = Some("Mogcord".to_string());
        client_options.connect_timeout = Some(Duration::from_secs(30));
        client_options.compressors = Some(vec![
            Compressor::Snappy,
            Compressor::Zlib {
                level: Default::default(),
            },
            Compressor::Zstd {
                level: Default::default(),
            },
        ]);
    
        let client = Client::with_options(client_options)?;

        let db = client.database("db_mogcord");

        let users: Collection<MongolUser> = db.collection("users");

        Ok(Self { users : users })
    }

    pub async fn get_user_db_object_by_id(&self, user_id: &String) -> Result<MongolUser, UserError>
    {
        let user_option = self
                                              .users
                                              .find_one(doc! { "uuid" : user_id }, None)
                                              .await
                                              .map_err(|err| UserError::UnexpectedError(Some(err.to_string())))?;

        match user_option
        {
            Some(user) => Ok(user),
            None => Err(UserError::UserNotFound)
        }
    }
}

#[async_trait]
impl UserRepository for MongolDB
{
    async fn does_user_exist_by_id(&self, user_id: &String) -> Result<bool, UserError>
    {
        let user_uuid = Uuid::parse_str(user_id)
                              .map_err(|_| UserError::UserNotFound)?;

        match self.users.find_one(doc! { "_id" : user_uuid }, None).await
        {
            Ok(option) => Ok(option.is_some()),
            Err(err) => Err(UserError::UnexpectedError(Some(err.to_string())))
        }
    }

    async fn does_user_exist_by_mail(&self, user_mail: &String) -> Result<bool, UserError>
    {
        match self.users.find_one(doc! { "mail" : user_mail }, None).await
        {
            Ok(option) => Ok(option.is_some()),
            Err(err) => Err(UserError::UnexpectedError(Some(err.to_string())))
        }
    }

    async fn create_user(&self, user: User) -> Result<User, UserError>
    {
        let db_user = MongolUser::convert_to_db(&user)
                                  .await
                                  .map_err(|err| UserError::UnexpectedError(Some(err.to_string())))?;
        
        match self.users.insert_one(&db_user, None).await
        {
            Ok(_) => Ok(user),
            Err(err) => Err(UserError::UnexpectedError(Some(err.to_string()))),
        }
    }

    async fn get_user_by_id(&self, user_id: &String) -> Result<User, UserError>
    {
        let user_uuid = Uuid::parse_str(user_id)
                              .map_err(|_| UserError::UserNotFound)?;

        let user_option = self.users.find_one(doc! { "_id": user_uuid }, None)
                                         .await
                                         .map_err(|err| UserError::UnexpectedError(Some(err.to_string())))?;
        
        match user_option 
        {
            Some(user) => Ok(user.convert_to_domain()),
            None => Err(UserError::UserNotFound),
        }
    }
}