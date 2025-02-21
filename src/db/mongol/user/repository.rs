use axum::async_trait;
use bson::Document;
use futures_util::StreamExt;
use mongodb::bson::{
    doc,
    from_document,
    Uuid,
};

use crate::db::mongol::{
    helper,
    MongolDB,
    MongolUser,
    MongolUserVec,
};
use crate::model::user::{
    self,
    User,
};
use crate::model::{
    error,
    Pagination,
};
use crate::{
    bubble,
    map_mongo_key_to_string,
    server_error,
};

#[async_trait]
impl user::Repository for MongolDB
{
    async fn does_user_exist_by_id<'input, 'err>(
        &'input self,
        user_id: &'input str,
    ) -> error::Result<'err, bool>
    {
        let user_id_local =
            bubble!(helper::convert_domain_id_to_mongol(user_id))?;

        let filter = doc! { "_id" : user_id_local };

        internal_does_user_exist(self, filter).await
    }

    async fn does_user_exist_by_mail<'input, 'err>(
        &'input self,
        user_mail: &'input str,
    ) -> error::Result<'err, bool>
    {
        let filter = doc! { "email" : user_mail };

        internal_does_user_exist(self, filter).await
    }

    async fn does_user_exist_by_username<'input, 'err>(
        &'input self,
        username: &'input str,
    ) -> error::Result<'err, bool>
    {
        let filter = doc! { "username" : username };

        internal_does_user_exist(self, filter).await
    }

    async fn create_user<'input, 'err>(
        &'input self,
        user: User,
    ) -> error::Result<'err, User>
    {
        let db_user = bubble!(MongolUser::try_from(&user))?;

        match self.users().insert_one(&db_user).await
        {
            Ok(_) => Ok(user),
            Err(err) => Err(server_error!(
                error::Kind::Insert,
                error::OnType::User
            )
            .add_debug_info("error", err.to_string())),
        }
    }

    async fn create_users<'input, 'err>(
        &'input self,
        users: Vec<User>,
    ) -> error::Result<'err, ()>
    {
        let db_users = bubble!(MongolUserVec::try_from(
            &users
        ))?;

        match self.users().insert_many(&db_users.0).await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(server_error!(
                error::Kind::Insert,
                error::OnType::User
            )
            .add_debug_info("error", err.to_string())),
        }
    }

    async fn get_user_by_id<'input, 'err>(
        &'input self,
        user_id: &'input str,
    ) -> error::Result<'err, User>
    {
        let user_id_local =
            bubble!(helper::convert_domain_id_to_mongol(user_id))?;

        let filter = doc! { "_id": user_id_local };

        internal_get_user(self, filter).await.map_err(|err| {
            server_error!(err).add_debug_info("user id", user_id.to_string())
        })
    }

    async fn get_user_by_mail<'input, 'err>(
        &'input self,
        email: &'input str,
    ) -> error::Result<'err, User>
    {
        let filter = doc! { "email": email };

        internal_get_user(self, filter).await.map_err(|err| {
            server_error!(err).add_debug_info("email", email.to_string())
        })
    }

    async fn get_users_by_id<'input, 'err>(
        &'input self,
        user_ids: Vec<String>,
    ) -> error::Result<'err, Vec<User>>
    {
        let mut user_ids_local: Vec<Uuid> = Vec::new();

        for user_id in user_ids
        {
            let user_id =
                bubble!(helper::convert_domain_id_to_mongol(&user_id))?;

            user_ids_local.push(user_id);
        }

        let pipelines = vec![
            doc! {
                "$match":
                {
                    "_id": { "$in": user_ids_local },
                }
            },
            //rename fields
            doc! {
                "$addFields":
                {
                    "id": map_mongo_key_to_string!("$_id", "uuid"),
                }
            },
            //hide fields
            doc! {
                "$unset": ["_id"]
            },
        ];

        let mut cursor =
            self.users().aggregate(pipelines).await.map_err(|err| {
                server_error!(
                    error::Kind::Fetch,
                    error::OnType::User
                )
                .add_debug_info("error", err.to_string())
            })?;

        let mut users: Vec<User> = Vec::new();

        while let Some(result) = cursor.next().await
        {
            match result
            {
                Ok(doc) =>
                {
                    let user: User = from_document(doc).map_err(|err| {
                        server_error!(
                            error::Kind::Parse,
                            error::OnType::User
                        )
                        .add_debug_info("error", err.to_string())
                    })?;
                    users.push(user);
                },
                Err(err) => println!("{err}"),
            }
        }

        Ok(users)
    }

    async fn get_users<'input, 'err>(
        &'input self,
        pagination: Pagination,
    ) -> error::Result<'err, Vec<User>>
    {
        let pipelines = vec![
            //rename fields
            doc! {
                "$addFields":
                {
                    "id": map_mongo_key_to_string!("$_id", "uuid"),
                }
            },
            //hide fields
            doc! {
                "$unset": ["_id"]
            },
            //skip offset
            doc! {
                "$skip": i32::try_from(pagination.get_skip_size()).ok().unwrap_or(0)
            },
            //limit output
            doc! {
                "$limit": i32::try_from(pagination.page_size).ok().unwrap_or(0)
            },
        ];

        let mut cursor =
            self.users().aggregate(pipelines).await.map_err(|err| {
                server_error!(
                    error::Kind::Fetch,
                    error::OnType::User
                )
                .add_debug_info("error", err.to_string())
            })?;

        let mut users: Vec<User> = Vec::new();

        while let Some(result) = cursor.next().await
        {
            match result
            {
                Ok(doc) =>
                {
                    let user: User = from_document(doc).map_err(|err| {
                        server_error!(
                            error::Kind::Parse,
                            error::OnType::User
                        )
                        .add_debug_info("error", err.to_string())
                    })?;
                    users.push(user);
                },
                Err(err) => println!("{err}"),
            }
        }

        Ok(users)
    }
}

async fn internal_does_user_exist<'input, 'err>(
    repo: &MongolDB,
    filter: Document,
) -> error::Result<'err, bool>
{
    match repo.users().find_one(filter).await
    {
        Ok(option) => Ok(option.is_some()),
        Err(err) => Err(server_error!(
            error::Kind::Fetch,
            error::OnType::User
        )
        .add_debug_info("error", err.to_string())),
    }
}

async fn internal_get_user<'input, 'err>(
    repo: &MongolDB,
    filter: Document,
) -> error::Result<'err, User>
{
    let user_option = repo.users().find_one(filter).await.map_err(|err| {
        server_error!(
            error::Kind::Fetch,
            error::OnType::User
        )
        .add_debug_info("error", err.to_string())
    })?;

    match user_option
    {
        Some(user) => Ok(User::from(&user)),
        None => Err(server_error!(
            error::Kind::NotFound,
            error::OnType::User
        )),
    }
}

fn _internal_wrap_valid_user_filter(filter: Document) -> Document
{
    doc! {
        "$and":
        [
            filter,
            { "flag": _internal_valid_user_filter() },
        ]
    }
}

fn _internal_valid_user_filter() -> Document
{
    doc! { "$in": [user::Flag::None, user::Flag::Admin, user::Flag::Owner] }
}
