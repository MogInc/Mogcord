use bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::{db::mongoldb::mongol_helper, model::token::RefreshToken};

use super::MongolError;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolRefreshToken
{
    pub token_value: String,
    pub device_id: Uuid,
}

impl TryFrom<&RefreshToken> for MongolRefreshToken
{
    type Error = MongolError;

    fn try_from(value: &RefreshToken) -> Result<Self, Self::Error> 
    {
        let device_id = mongol_helper::convert_domain_id_to_mongol(&value.device_id)?;

        Ok(
            Self
            {
                token_value: value.value.clone(),
                device_id,
            }
        )
    }
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