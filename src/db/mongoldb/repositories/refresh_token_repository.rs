use axum::async_trait;
use bson::{doc, from_document, DateTime, Document};
use futures_util::StreamExt;

use crate::{convert_mongo_key_to_string, db::mongoldb::{mongol_helper, MongolDB, MongolRefreshToken}, model::{misc::ServerError, token::{RefreshToken, RefreshTokenFlag, RefreshTokenRepository}}};

#[async_trait]
impl RefreshTokenRepository for MongolDB
{
    async fn create_token(&self, token: RefreshToken) -> Result<RefreshToken, ServerError>
    {
        let db_token = MongolRefreshToken::try_from(&token)?;
        
        match self.refresh_tokens().insert_one(&db_token).await
        {
            Ok(_) => Ok(token),
            Err(err) => Err(ServerError::FailedInsert(err.to_string())),
        }
    }

    async fn get_valid_token_by_device_id(&self, device_id: &str) -> Result<RefreshToken, ServerError>
    {
        let device_id_local = mongol_helper::convert_domain_id_to_mongol(&device_id)?;

        let pipelines = vec![
            //filter
            doc! 
            {
                "$match":
                {
                    "device_id": device_id_local,
                    "expiration_date": { "$gte": DateTime::now() },
                    "flag": valid_refresh_token_filter(),
                }
            },
            //join with owners
            doc! 
            {
                "$lookup":
                {
                    "from": "users",
                    "localField": "owner_id",
                    "foreignField": "_id",
                    "as": "owner"
                },
            },
            //join with users
            doc! 
            {
                "$unwind":
                {
                    "path": "$owner"
                },
            },
            //rename fields
            doc!
            {
                "$addFields":
                {
                    "device_id": convert_mongo_key_to_string!("$device_id", "uuid"),
                    "owner.id": convert_mongo_key_to_string!("$owner._id", "uuid"),
                }
            },
            //hide fields
            doc! 
            {
                "$unset": ["_id", "owner_id", "owner._id"]
            },
        ];

        let mut cursor = self
            .refresh_tokens()
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
                let refresh_token = from_document(document)
                    .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

                return Ok(refresh_token);
            },
            None => Err(ServerError::RefreshTokenNotFound), 
        }
    }

    async fn revoke_token(&self, user_id: &str, device_id: &str) -> Result<(), ServerError>
    {
        let user_id_local = mongol_helper::convert_domain_id_to_mongol(&user_id)?;

        let device_id_local = mongol_helper::convert_domain_id_to_mongol(&device_id)?;

        let filter = doc!
        {
            "owner_id": user_id_local,
            "device_id": device_id_local,
        };

        let update = doc!
        {
            "$set": { "flag": RefreshTokenFlag::Revoked }
        };

        match self.refresh_tokens().update_one(filter, update).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(ServerError::FailedUpdate(err.to_string())),
        }
    }
    async fn revoke_all_tokens(&self, user_id: &str) -> Result<(), ServerError>
    {
        let user_id_local = mongol_helper::convert_domain_id_to_mongol(&user_id)?;

        let filter = doc!
        {
            "owner_id": user_id_local,
            "flag": valid_refresh_token_filter(),
            "expiration_date": { "$gte": DateTime::now() },
        };

        let update = doc!
        {
            "$set": { "flag": RefreshTokenFlag::Revoked }
        };

        match self.refresh_tokens().update_many(filter, update).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(ServerError::FailedUpdate(err.to_string())),
        }
    }
}

fn valid_refresh_token_filter() -> Document
{
    doc! { "$in": [ RefreshTokenFlag::None ] }
}