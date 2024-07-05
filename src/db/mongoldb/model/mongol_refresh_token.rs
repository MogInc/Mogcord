use bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::model::token::RefreshToken;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolRefreshToken
{
    pub token_value: String,
    pub device_id: Uuid,
}

impl From<&MongolRefreshToken> for RefreshToken
{
    fn from(value: &MongolRefreshToken) -> Self 
    {
        return RefreshToken::new(
            value.token_value.clone(),
            value.device_id.to_string(),
        );
    }
}