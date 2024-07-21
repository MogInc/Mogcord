use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};

use crate::model::AppState;
use crate::middleware::auth::Ctx;
use crate::dto::{ObjectToDTO, UserGetResponse};


pub async fn get_ctx_user_auth(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
) -> impl IntoResponse
{
    let repo_user = &state.user;
 
    let ctx_user_id = &ctx.user_id_ref();
    
    match repo_user.get_user_by_id(ctx_user_id).await 
    {
        Ok(user) => Ok(Json(UserGetResponse::obj_to_dto(user))),
        Err(e) => Err(e),
    }
}