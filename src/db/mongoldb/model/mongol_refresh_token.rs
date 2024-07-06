use bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::{db::mongoldb::mongol_helper, model::{token::RefreshToken, user::User}};

use super::{MongolError, MongolUser};

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolRefreshToken
{
    pub token_value: String,
    pub device_id: Uuid,
    pub owner: MongolUser,
}

impl TryFrom<(&RefreshToken, &User)> for MongolRefreshToken
{
    type Error = MongolError;

    fn try_from(value: (&RefreshToken, &User)) -> Result<Self, Self::Error> 
    {
        let device_id = mongol_helper::convert_domain_id_to_mongol(&value.0.device_id)?;
        let user = MongolUser::try_from(value.1)?;
        Ok(
            Self
            {
                token_value: value.0.value.clone(),
                device_id,
                owner: user,
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