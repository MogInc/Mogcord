use axum::async_trait;
use crate::{db::mongoldb::MongolDB, model::{misc::ServerError, relation::RelationRepository}};
use crate::db::mongoldb::mongol_helper;
use mongodb::bson::doc;

#[async_trait]
impl RelationRepository for MongolDB
{
    async fn does_friendship_exist(&self, current_user_id: &str, other_user_id: &str) -> Result<bool, ServerError>
    {
        let current_user_id_local = mongol_helper::convert_domain_id_to_mongol(&current_user_id)
            .map_err(|_| ServerError::UserNotFound)?;

        let other_user_id_local = mongol_helper::convert_domain_id_to_mongol(&other_user_id)
            .map_err(|_| ServerError::UserNotFound)?;

        let filter = doc!
        {
            "$and":
            [
                doc! { "user_id" : current_user_id_local },
                doc! { "friend_ids" : other_user_id_local },
            ]
        };

        match self.relations().find_one(filter).await
        {
            Ok(option) => Ok(option.is_some()),
            Err(err) => Err(ServerError::FailedRead(err.to_string()))
        }
    }

    async fn add_friend(&self, current_user_id: &str, other_user_id: &str) -> Result<(), ServerError>
    {
        let current_user_id_local = mongol_helper::convert_domain_id_to_mongol(&current_user_id)
        .map_err(|_| ServerError::UserNotFound)?;

        let other_user_id_local = mongol_helper::convert_domain_id_to_mongol(&other_user_id)
            .map_err(|_| ServerError::UserNotFound)?;

        let filter = doc!
        {
            "$and":
            [
                doc! { "user_id" : current_user_id_local },
                doc! { "friend_ids" : other_user_id_local },
            ]
        };

        let update = doc! 
        {
            "$push": { "friend_ids": other_user_id_local}
        };

        match self.relations().update_one(filter, update).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(ServerError::FailedRead(err.to_string()))
        }
    }
}