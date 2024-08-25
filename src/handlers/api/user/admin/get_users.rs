use axum::extract::{
    Query,
    State,
};
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use crate::dto::{
    vec_to_dto,
    UserGetResponse,
};
use crate::model::user::User;
use crate::model::{
    AppState,
    Pagination,
};

pub async fn get_users(
    State(state): State<Arc<AppState>>,
    pagination: Option<Query<Pagination>>,
) -> impl IntoResponse
{
    let repo_user = &state.users;

    let pagination = Pagination::new(pagination);

    match repo_user.get_users(pagination).await
    {
        Ok(users) => Ok(Json(vec_to_dto::<
            User,
            UserGetResponse,
        >(users))),
        Err(e) => Err(e),
    }
}
