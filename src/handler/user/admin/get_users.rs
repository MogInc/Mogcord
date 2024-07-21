use std::sync::Arc;
use axum::{extract::{Query, State}, response::IntoResponse, Json};

use crate::model::{AppState, Pagination};
use crate::dto::{vec_to_dto, UserGetResponse};
use crate::model::user::User;


pub async fn get_users(
    State(state): State<Arc<AppState>>,
    pagination: Option<Query<Pagination>>,
) -> impl IntoResponse
{
    let repo_user = &state.user;

    let pagination = Pagination::new(pagination);

    match repo_user.get_users(pagination).await 
    {
        Ok(users) => Ok(Json(vec_to_dto::<User, UserGetResponse>(users))),
        Err(e) => Err(e),
    }
}