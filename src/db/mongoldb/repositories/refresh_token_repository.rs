use axum::async_trait;
use bson::doc;

use crate::{db::mongoldb::{mongol_helper, MongolDB, MongolRefreshToken}, model::{misc::ServerError, token::{RefreshToken, RefreshTokenRepository}, user::User}};

#[async_trait]
impl RefreshTokenRepository for MongolDB
{
    async fn create_token(&self, token: RefreshToken, owner: &User) -> Result<RefreshToken, ServerError>
    {
        let db_token = MongolRefreshToken::try_from((&token, owner))
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;
        
        match self.refresh_tokens().insert_one(&db_token).await
        {
            Ok(_) => Ok(token),
            Err(err) => Err(ServerError::FailedInsert(err.to_string())),
        }
    }

    async fn get_token_by_device_id(&self, device_id: &str) -> Result<RefreshToken, ServerError>
    {
        let device_id_local = mongol_helper::convert_domain_id_to_mongol(&device_id)
            .map_err(|_| ServerError::RefreshTokenNotFound)?;

        let token_option = self
            .refresh_tokens()
            .find_one(doc! { "device_id" : device_id_local })
            .await
            .map_err(|err| ServerError::FailedRead(err.to_string()))?;

        match token_option
        {
            Some(token) => Ok(RefreshToken::from(&token)),
            None => Err(ServerError::RefreshTokenNotFound)
        }
    }
}