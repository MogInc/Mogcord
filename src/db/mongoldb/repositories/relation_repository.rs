use axum::async_trait;
use bson::{Document, Uuid};
use crate::{db::mongoldb::{MongolDB, MongolRelation}, model::{misc::ServerError, relation::RelationRepository}};
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
                doc! 
                {
                    "$or" : 
                    [
                        { "pending_outgoing_friend_ids" : other_user_id_local },
                        { "friend_ids" : other_user_id_local },
                    ],
                }
            ]
        };

        does_user_relation_exist(self, filter).await
    }

    async fn does_incoming_friendship_exist(&self, current_user_id: &str, other_user_id: &str) -> Result<bool, ServerError>
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
                doc! { "pending_incoming_friend_ids": other_user_id_local }
            ]
        };

        does_user_relation_exist(self, filter).await
    }

    async fn does_blocked_exist(&self, current_user_id: &str, other_user_id: &str) -> Result<bool, ServerError>
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
                doc! { "blocked_ids" : other_user_id_local },
            ]
        };

        does_user_relation_exist(self, filter).await
    }

    async fn add_user_as_friend(&self, current_user_id: &str, other_user_id: &str) -> Result<(), ServerError>
    {
        let current_user_id_local = mongol_helper::convert_domain_id_to_mongol(&current_user_id)
            .map_err(|_| ServerError::UserNotFound)?;

        let other_user_id_local = mongol_helper::convert_domain_id_to_mongol(&other_user_id)
            .map_err(|_| ServerError::UserNotFound)?;


        let mut session = self
            .client()
            .start_session()            
            .await
            .map_err(|err| ServerError::TransactionError(err.to_string()))?;

        session
            .start_transaction()
            .await
            .map_err(|err| ServerError::TransactionError(err.to_string()))?;


        let filter_current_user = doc! { "user_id" : current_user_id_local };
        let update_current_user = doc! 
        {
            "$push": { "pending_outgoing_friend_ids": other_user_id_local }
        };
        

        let filter_other_user = doc! { "user_id" : other_user_id_local };
        let update_other_user = doc! 
        {
            "$push": { "pending_incoming_friend_ids": current_user_id_local }
        };

        add_relation(self, current_user_id_local).await?;
        add_relation(self, other_user_id_local).await?;

        self
            .relations()
            .update_one(filter_other_user, update_other_user)
            .session(&mut session)
            .await
            .map_err(|err| ServerError::FailedUpdate(err.to_string()))?;

        //can remove this match and have implicit abort
        match self.relations().update_one(filter_current_user, update_current_user).session(&mut session).await
        {
            Ok(_) => 
            {
                session
                    .commit_transaction()
                    .await
                    .map_err(|err| ServerError::TransactionError(err.to_string()))?;

                Ok(())
            },
            Err(err) => 
            {
                session
                    .abort_transaction()
                    .await
                    .map_err(|err| ServerError::TransactionError(err.to_string()))?;

                Err(ServerError::FailedUpdate(err.to_string()))
            }
        }
    }

    async fn add_user_as_blocked(&self, current_user_id: &str, other_user_id: &str) -> Result<(), ServerError>
    {
        let current_user_id_local = mongol_helper::convert_domain_id_to_mongol(&current_user_id)
            .map_err(|_| ServerError::UserNotFound)?;

        let other_user_id_local = mongol_helper::convert_domain_id_to_mongol(&other_user_id)
            .map_err(|_| ServerError::UserNotFound)?;

        let mut session = self
            .client()
            .start_session()
            .await
            .map_err(|err| ServerError::TransactionError(err.to_string()))?;

        session
            .start_transaction()
            .await
            .map_err(|err| ServerError::TransactionError(err.to_string()))?;


        let filter_current_user = doc! { "user_id" : current_user_id_local };
        let update_current_user = doc! 
        {
            "$push": { "blocked_ids": other_user_id_local },
            "$pull": 
            { 
                "friend_ids": other_user_id_local,
                "pending_incoming_friend_ids": other_user_id_local,
                "pending_outgoing_friend_ids": other_user_id_local
            },
        };

        
        let filter_other_user = doc! { "user_id" : other_user_id_local };
        let update_other_user = doc! 
        {
            "$pull": 
            { 
                "friend_ids": current_user_id_local,
                "pending_incoming_friend_ids": current_user_id_local,
                "pending_outgoing_friend_ids": current_user_id_local
            },
        };

        add_relation(self, current_user_id_local).await?;
        add_relation(self, other_user_id_local).await?;

        self
            .relations()
            .update_one(filter_other_user, update_other_user)
            .session(&mut session)
            .await
            .map_err(|err| ServerError::FailedUpdate(err.to_string()))?;
       
       //can remove this match and have implicit abort
        match self.relations().update_one(filter_current_user, update_current_user).session(&mut session).await
        {
            Ok(_) => 
            {
                session
                    .commit_transaction()
                    .await
                    .map_err(|err| ServerError::TransactionError(err.to_string()))?;

                Ok(())
            },
            Err(err) => 
            {
                session
                    .abort_transaction()
                    .await
                    .map_err(|err| ServerError::TransactionError(err.to_string()))?;
                
                Err(ServerError::FailedUpdate(err.to_string()))
            }
        }
    }

    async fn confirm_user_as_friend(&self, current_user_id: &str, other_user_id: &str) -> Result<(), ServerError>
    {
        let current_user_id_local = mongol_helper::convert_domain_id_to_mongol(&current_user_id)
        .map_err(|_| ServerError::UserNotFound)?;

        let other_user_id_local = mongol_helper::convert_domain_id_to_mongol(&other_user_id)
            .map_err(|_| ServerError::UserNotFound)?;


        let mut session = self
            .client()
            .start_session()
            .await
            .map_err(|err| ServerError::TransactionError(err.to_string()))?;

        session
            .start_transaction()
            .await
            .map_err(|err| ServerError::TransactionError(err.to_string()))?;
        
        let filter_current_user = doc!{ "user_id": current_user_id_local };
        let update_current_user = doc! 
        {
            "$push": { "friend_ids": other_user_id_local },
            "$pull": { "pending_incoming_friend_ids": other_user_id_local },
        };
        
        
        let filter_other_user = doc!{ "user_id": other_user_id_local };
        let update_other_user = doc! 
        {
            "$push": { "friend_ids": current_user_id_local },
            "$pull": { "pending_outgoing_friend_ids": current_user_id_local },
        };


        self
            .relations()
            .update_one(filter_other_user, update_other_user)
            .session(&mut session)
            .await
            .map_err(|err| ServerError::FailedUpdate(err.to_string()))?;

        //can remove this match and have implicit abort
        match self.relations().update_one(filter_current_user, update_current_user).session(&mut session).await
        {
            Ok(_) => 
            {
                session
                    .commit_transaction()
                    .await
                    .map_err(|err| ServerError::TransactionError(err.to_string()))?;

                Ok(())
            }
            Err(err) => 
            {
                session
                    .abort_transaction()
                    .await
                    .map_err(|err| ServerError::TransactionError(err.to_string()))?;

                Err(ServerError::FailedUpdate(err.to_string()))
            }
        }
    }

    async fn remove_user_as_friend(&self, current_user_id: &str, other_user_id: &str) -> Result<(), ServerError>
    {
        let current_user_id_local = mongol_helper::convert_domain_id_to_mongol(&current_user_id)
            .map_err(|_| ServerError::UserNotFound)?;

        let other_user_id_local = mongol_helper::convert_domain_id_to_mongol(&other_user_id)
            .map_err(|_| ServerError::UserNotFound)?;

        let filter = doc! { "user_id" : current_user_id_local };

        let update = doc! 
        {
            "$pull": 
            { 
                "friend_ids": other_user_id_local,
                "pending_outgoing_friend_ids": other_user_id_local,
            },
        };

        match self.relations().update_one(filter, update).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(ServerError::FailedUpdate(err.to_string()))
        }
    }

    async fn remove_user_as_blocked(&self, current_user_id: &str, other_user_id: &str) -> Result<(), ServerError>
    {
        let current_user_id_local = mongol_helper::convert_domain_id_to_mongol(&current_user_id)
            .map_err(|_| ServerError::UserNotFound)?;

        let other_user_id_local = mongol_helper::convert_domain_id_to_mongol(&other_user_id)
            .map_err(|_| ServerError::UserNotFound)?;

        let filter = doc! { "user_id" : current_user_id_local };

        let update = doc! 
        {
            "$pull": { "blocked_ids": other_user_id_local },
        };

        match self.relations().update_one(filter, update).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(ServerError::FailedUpdate(err.to_string()))
        }
    }
}

async fn add_relation(repo: &MongolDB, current_user_id: Uuid) -> Result<(), ServerError>
{
    let filter = doc! { "user_id" : current_user_id };

    let relation_option = repo
        .relations()
        .find_one(filter)
        .await
        .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

    if let None = relation_option
    {
        let relation = MongolRelation::new(current_user_id);

        repo
            .relations()
            .insert_one(relation)
            .await
            .map_err(|err| ServerError::FailedInsert(err.to_string()))?;
    }

    Ok(())
}

async fn does_user_relation_exist(repo: &MongolDB, filter: Document) -> Result<bool, ServerError>
{
    match repo.relations().find_one(filter).await
    {
        Ok(option) => Ok(option.is_some()),
        Err(err) => Err(ServerError::FailedRead(err.to_string()))
    }
}