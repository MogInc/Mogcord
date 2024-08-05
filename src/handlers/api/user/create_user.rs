use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};

use crate::handlers::logic;
use crate::handlers::logic::user::CreateUserRequest;
use crate::model::AppState;
use crate::dto::{ObjectToDTO, UserCreateResponse};


pub async fn create_user(
    State(state): State<Arc<AppState>>, 
    Json(payload): Json<CreateUserRequest>
) -> impl IntoResponse
{
    match logic::user::create_user(&state, &payload).await 
    {
        Ok(user) => Ok(Json(UserCreateResponse::obj_to_dto(user))),
        Err(err) => Err(err),
    }
}