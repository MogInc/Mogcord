use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use crate::dto::{ObjectToDTO, UserCreateResponse};
use crate::handlers::logic;
use crate::handlers::logic::user::CreateUserRequest;
use crate::model::AppState;

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse
{
    match logic::user::create_user(&state, &payload).await
    {
        Ok(user) => Ok(Json(
            UserCreateResponse::obj_to_dto(user),
        )),
        Err(err) => Err(err),
    }
}
