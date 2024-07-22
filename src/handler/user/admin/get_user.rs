use std::sync::Arc;
use axum::{extract::{Path, State}, response::IntoResponse, Json};

use crate::model::AppState;
use crate::dto::{ObjectToDTO, UserGetResponse};


pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>
) -> impl IntoResponse
{   
    let repo_user = &state.users;

    match repo_user.get_user_by_id(&user_id).await 
    {
        Ok(user) => Ok(Json(UserGetResponse::obj_to_dto(user))),
        Err(e) => Err(e),
    }
}
