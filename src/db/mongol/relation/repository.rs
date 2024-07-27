use axum::async_trait;
use bson::{Document, Uuid};
use mongodb::bson::doc;

use crate::model::{error, relation};
use crate::db::mongol::{MongolDB, MongolRelation};
use crate::db::mongol::helper;

#[async_trait]
impl relation::Repository for MongolDB
{
    async fn does_friendship_exist<'input, 'err>(
        &'input self, 
        current_user_id: &'input str, 
        other_user_id: &'input str
    ) -> Result<bool, error::Server<'err>>
    {
        let current_user_id_local = helper::convert_domain_id_to_mongol(current_user_id)?;

        let other_user_id_local = helper::convert_domain_id_to_mongol(other_user_id)?;

        let filter = doc!
        {
            "$and":
            [
                doc! { "user_id" : current_user_id_local },
                doc! { "friend_ids" : other_user_id_local },
            ]
        };

        does_user_relation_exist(self, filter).await
    }

    async fn does_friendships_exist<'input, 'err>(
        &'input self, 
        current_user_id: &'input str, 
        other_user_ids: Vec<&'input str>
    ) -> Result<bool, error::Server<'err>>
    {
        let current_user_id_local = helper::convert_domain_id_to_mongol(current_user_id)?;

        let other_user_ids_local = helper::convert_domain_ids_to_mongol(&other_user_ids)?;

        let filter = doc!
        {
            "user_id" : current_user_id_local,
        };

        let mongol_relation_option = self
            .relations()
            .find_one(filter)
            .await
            .map_err(|err| error::Server::new(
                error::Kind::Fetch,
                error::OnType::RelationFriend,
                file!(),
                line!())
                .add_debug_info(err.to_string())
            )?;


        match mongol_relation_option
        {
            Some(mongol_relation) => 
            {
                let is_all_friends = mongol_relation
                    .friend_ids
                    .iter()
                    .any(|id| !other_user_ids_local.contains(id));

                Ok(is_all_friends)
            },
            None => Ok(other_user_ids_local.is_empty()),
        }
    }

    async fn does_outgoing_friendship_exist<'input, 'err>(
        &'input self, 
        current_user_id: &'input str, 
        other_user_id: &'input str
    ) -> Result<bool, error::Server<'err>>
    {
        let current_user_id_local = helper::convert_domain_id_to_mongol(current_user_id)?;

        let other_user_id_local = helper::convert_domain_id_to_mongol(other_user_id)?;

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

    async fn does_incoming_friendship_exist<'input, 'err>(
        &'input self, 
        current_user_id: &'input str,
         other_user_id: &'input str
    ) -> Result<bool, error::Server<'err>>
    {
        let current_user_id_local = helper::convert_domain_id_to_mongol(current_user_id)?;

        let other_user_id_local = helper::convert_domain_id_to_mongol(other_user_id)?;

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

    async fn does_blocked_exist<'input, 'err>(
        &'input self, 
        current_user_id: &'input str, 
        other_user_id: &'input str
    ) -> Result<bool, error::Server<'err>>
    {
        let current_user_id_local = helper::convert_domain_id_to_mongol(current_user_id)?;

        let other_user_id_local = helper::convert_domain_id_to_mongol(other_user_id)?;

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

    async fn add_user_as_friend<'input, 'err>(
        &'input self, 
        current_user_id: &'input str,
         other_user_id: &'input str
    ) -> Result<(), error::Server<'err>>
    {
        let current_user_id_local = helper::convert_domain_id_to_mongol(current_user_id)?;

        let other_user_id_local = helper::convert_domain_id_to_mongol(other_user_id)?;


        let mut session = self
            .client()
            .start_session()            
            .await
            .map_err(|err| error::map_transaction(&err, file!(), line!()))?;

        session
            .start_transaction()
            .await
            .map_err(|err| error::map_transaction(&err, file!(), line!()))?;


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
            .map_err(|err| error::Server::new(
                error::Kind::Update,
                error::OnType::RelationFriend,
                file!(),
                line!())
                .add_debug_info(err.to_string())
            )?;

        //can remove this match and have implicit abort
        match self.relations().update_one(filter_current_user, update_current_user).session(&mut session).await
        {
            Ok(_) => 
            {
                session
                    .commit_transaction()
                    .await
                    .map_err(|err| error::map_transaction(&err, file!(), line!()))?;

                Ok(())
            },
            Err(err) => 
            {
                session
                    .abort_transaction()
                    .await
                    .map_err(|err| error::map_transaction(&err, file!(), line!()))?;

                Err(error::Server::new(
                    error::Kind::Update, 
                    error::OnType::RelationFriend, 
                    file!(), 
                    line!())
                    .add_debug_info(err.to_string())
                )
            }
        }
    }

    async fn add_user_as_blocked<'input, 'err>(
        &'input self, 
        current_user_id: &'input str, 
        other_user_id: &'input str
    ) -> Result<(), error::Server<'err>>
    {
        let current_user_id_local = helper::convert_domain_id_to_mongol(current_user_id)?;

        let other_user_id_local = helper::convert_domain_id_to_mongol(other_user_id)?;

        let mut session = self
            .client()
            .start_session()
            .await
            .map_err(|err| error::map_transaction(&err, file!(), line!()))?;

        session
            .start_transaction()
            .await
            .map_err(|err| error::map_transaction(&err, file!(), line!()))?;

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
            .map_err(|err| error::Server::new(
                error::Kind::Update,
                error::OnType::RelationBlocked,
                file!(),
                line!())
                .add_debug_info(err.to_string())
            )?;
       
       //can remove this match and have implicit abort
        match self.relations().update_one(filter_current_user, update_current_user).session(&mut session).await
        {
            Ok(_) => 
            {
                session
                    .commit_transaction()
                    .await
                    .map_err(|err| error::map_transaction(&err, file!(), line!()))?;

                Ok(())
            },
            Err(err) => 
            {
                session
                    .abort_transaction()
                    .await
                    .map_err(|err| error::map_transaction(&err, file!(), line!()))?;
                
                Err(error::Server::new(
                    error::Kind::Update, 
                    error::OnType::RelationBlocked, 
                    file!(), 
                    line!())
                    .add_debug_info(err.to_string())
                )
            }
        }
    }

    async fn confirm_user_as_friend<'input, 'err>(
        &'input self, current_user_id: &'input str, 
        other_user_id: &'input str
    ) -> Result<(), error::Server<'err>>
    {
        let current_user_id_local = helper::convert_domain_id_to_mongol(current_user_id)?;

        let other_user_id_local = helper::convert_domain_id_to_mongol(other_user_id)?;


        let mut session = self
            .client()
            .start_session()
            .await
            .map_err(|err| error::map_transaction(&err, file!(), line!()))?;

        session
            .start_transaction()
            .await
            .map_err(|err| error::map_transaction(&err, file!(), line!()))?;

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
            .map_err(|err| error::Server::new(
                error::Kind::Update,
                error::OnType::RelationFriend,
                file!(),
                line!())
                .add_debug_info(err.to_string())
            )?;

        //can remove this match and have implicit abort
        match self.relations().update_one(filter_current_user, update_current_user).session(&mut session).await
        {
            Ok(_) => 
            {
                session
                    .commit_transaction()
                    .await
                    .map_err(|err| error::map_transaction(&err, file!(), line!()))?;

                Ok(())
            }
            Err(err) => 
            {
                session
                    .abort_transaction()
                    .await
                    .map_err(|err| error::map_transaction(&err, file!(), line!()))?;

                Err(error::Server::new(
                    error::Kind::Update, 
                    error::OnType::RelationFriend, 
                    file!(), 
                    line!())
                    .add_debug_info(err.to_string())
                )
            }
        }
    }

    async fn remove_user_as_friend<'input, 'err>(
        &'input self, current_user_id: &'input str, 
        other_user_id: &'input str
    ) -> Result<(), error::Server<'err>>
    {
        let current_user_id_local = helper::convert_domain_id_to_mongol(current_user_id)?;

        let other_user_id_local = helper::convert_domain_id_to_mongol(other_user_id)?;

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
            Err(err) => Err(
                error::Server::new(
                    error::Kind::Delete, 
                    error::OnType::RelationFriend, 
                    file!(), 
                    line!())
                    .add_debug_info(err.to_string()
                )
            )
        }
    }

    async fn remove_user_as_blocked<'input, 'err>(
        &'input self, current_user_id: &'input str, 
        other_user_id: &'input str
    ) -> Result<(), error::Server<'err>>
    {
        let current_user_id_local = helper::convert_domain_id_to_mongol(current_user_id)?;

        let other_user_id_local = helper::convert_domain_id_to_mongol(other_user_id)?;

        let filter = doc! { "user_id" : current_user_id_local };

        let update = doc! 
        {
            "$pull": { "blocked_ids": other_user_id_local },
        };

        match self.relations().update_one(filter, update).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(
                error::Server::new(
                    error::Kind::Delete, 
                    error::OnType::RelationBlocked, 
                    file!(), 
                    line!())
                    .add_debug_info(err.to_string()
                )
            )
        }
    }
}

async fn add_relation<'err>(
    repo: &MongolDB, 
    current_user_id: Uuid
) -> Result<(), error::Server<'err>>
{
    let filter = doc! { "user_id" : current_user_id };

    let relation_option = repo
        .relations()
        .find_one(filter)
        .await
        .map_err(|err| error::Server::new(
            error::Kind::Fetch,
            error::OnType::Relation,
            file!(),
            line!())
            .add_debug_info(err.to_string())
        )?;

    if relation_option.is_none()
    {
        let relation = MongolRelation::new(current_user_id);

        repo
            .relations()
            .insert_one(relation)
            .await
            .map_err(|err| error::Server::new(
                error::Kind::Insert,
                error::OnType::Relation,
                file!(),
                line!())
                .add_debug_info(err.to_string())
            )?;
    }

    Ok(())
}

async fn does_user_relation_exist<'err>(
    repo: &MongolDB, 
    filter: Document
) -> Result<bool, error::Server<'err>>
{
    match repo.relations().find_one(filter).await
    {
        Ok(option) => Ok(option.is_some()),
        Err(err) => Err(
            error::Server::new(
                error::Kind::Fetch, 
                error::OnType::Relation, 
                file!(), 
                line!())
                .add_debug_info(err.to_string()
            )
        )
    }
}