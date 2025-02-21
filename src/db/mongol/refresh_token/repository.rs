use axum::async_trait;
use bson::{
    doc,
    from_document,
    DateTime,
    Document,
};
use futures_util::StreamExt;

use crate::db::mongol::{
    helper,
    MongolDB,
    MongolRefreshToken,
};
use crate::model::error;
use crate::model::refresh_token::{
    self,
    RefreshToken,
};
use crate::{
    bubble,
    map_mongo_key_to_string,
    server_error,
};

#[async_trait]
impl refresh_token::Repository for MongolDB
{
    async fn create_token<'input, 'err>(
        &'input self,
        token: RefreshToken,
    ) -> error::Result<'err, RefreshToken>
    {
        let db_token = bubble!(MongolRefreshToken::try_from(
            &token
        ))?;

        match self.refresh_tokens().insert_one(&db_token).await
        {
            Ok(_) => Ok(token),
            Err(err) => Err(server_error!(
                error::Kind::Insert,
                error::OnType::RefreshToken
            )
            .add_debug_info("error", err.to_string())),
        }
    }

    async fn get_valid_token<'input, 'err>(
        &'input self,
        device_id: &'input str,
        user_id: &'input str,
    ) -> error::Result<'err, RefreshToken>
    {
        let device_id_local =
            bubble!(helper::convert_domain_id_to_mongol(device_id))?;
        let user_id_local =
            bubble!(helper::convert_domain_id_to_mongol(user_id))?;

        let pipelines = vec![
            //filter
            doc! {
                "$match":
                {
                    "device_id": device_id_local,
                    "owner_id": user_id_local,
                    "expiration_date": { "$gte": DateTime::now() },
                    "flag": internal_valid_refresh_token_filter(),
                }
            },
            //join with owners
            doc! {
                "$lookup":
                {
                    "from": "users",
                    "localField": "owner_id",
                    "foreignField": "_id",
                    "as": "owner"
                },
            },
            //join with users
            doc! {
                "$unwind":
                {
                    "path": "$owner"
                },
            },
            //rename fields
            doc! {
                "$addFields":
                {
                    "device_id": map_mongo_key_to_string!("$device_id", "uuid"),
                    "owner.id": map_mongo_key_to_string!("$owner._id", "uuid"),
                }
            },
            //hide fields
            doc! {
                "$unset": ["_id", "owner_id", "owner._id"]
            },
        ];

        let mut cursor =
            self.refresh_tokens().aggregate(pipelines).await.map_err(
                |err| {
                    server_error!(
                        error::Kind::Fetch,
                        error::OnType::RefreshToken
                    )
                    .add_debug_info("error", err.to_string())
                },
            )?;

        let document_option =
            cursor.next().await.transpose().map_err(|err| {
                server_error!(
                    error::Kind::Unexpected,
                    error::OnType::RefreshToken
                )
                .add_debug_info("error", err.to_string())
            })?;

        match document_option
        {
            Some(document) =>
            {
                let refresh_token = from_document(document).map_err(|err| {
                    server_error!(
                        error::Kind::Parse,
                        error::OnType::RefreshToken
                    )
                    .add_debug_info("error", err.to_string())
                })?;

                Ok(refresh_token)
            },
            None => Err(server_error!(
                error::Kind::NotFound,
                error::OnType::RefreshToken
            )
            .add_debug_info(
                "device id",
                device_id.to_string(),
            )),
        }
    }

    async fn revoke_token<'input, 'err>(
        &'input self,
        user_id: &'input str,
        device_id: &'input str,
    ) -> error::Result<'err, ()>
    {
        let user_id_local =
            bubble!(helper::convert_domain_id_to_mongol(user_id))?;

        let device_id_local =
            bubble!(helper::convert_domain_id_to_mongol(device_id))?;

        let filter = doc! {
            "owner_id": user_id_local,
            "device_id": device_id_local,
        };

        let update = doc! {
            "$set": { "flag": refresh_token::Flag::Revoked }
        };

        match self.refresh_tokens().update_one(filter, update).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(server_error!(
                error::Kind::Revoke,
                error::OnType::RefreshToken
            )
            .add_debug_info("error", err.to_string())),
        }
    }

    async fn revoke_all_tokens<'input, 'err>(
        &'input self,
        user_id: &'input str,
    ) -> error::Result<'err, ()>
    {
        let user_id_local =
            bubble!(helper::convert_domain_id_to_mongol(user_id))?;

        let filter = doc! {
            "owner_id": user_id_local,
            "flag": internal_valid_refresh_token_filter(),
            "expiration_date": { "$gte": DateTime::now() },
        };

        let update = doc! {
            "$set": { "flag": refresh_token::Flag::Revoked }
        };

        match self.refresh_tokens().update_many(filter, update).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(server_error!(
                error::Kind::Revoke,
                error::OnType::RefreshToken
            )
            .add_debug_info("error", err.to_string())),
        }
    }

    async fn update_expiration<'input, 'err>(
        &'input self,
        token: &'input RefreshToken,
    ) -> error::Result<'err, ()>
    {
        let db_token = bubble!(MongolRefreshToken::try_from(
            token
        ))?;

        let filter = doc! {
            "device_id": db_token.device_id,
            "owner_id": db_token.owner_id,
            "expiration_date": { "$gte": DateTime::now() },
            "flag": internal_valid_refresh_token_filter(),
        };

        let update = doc! {
            "$set": { "expiration_date": db_token.expiration_date }
        };

        match self.refresh_tokens().update_one(filter, update).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(server_error!(
                error::Kind::Update,
                error::OnType::RefreshToken
            )
            .add_debug_info("error", err.to_string())),
        }
    }
}

fn internal_valid_refresh_token_filter() -> Document
{
    doc! { "$in": [ refresh_token::Flag::None ] }
}
