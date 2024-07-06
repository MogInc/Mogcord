use bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::{db::mongoldb::mongol_helper, model::token::RefreshToken};

use super::MongolError;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolRefreshToken
{
    pub value: String,
    pub device_id: Uuid,
    pub owner_id: Uuid,
}

impl TryFrom<&RefreshToken> for MongolRefreshToken
{
    type Error = MongolError;

    fn try_from(value: &RefreshToken) -> Result<Self, Self::Error> 
    {
        let device_id = mongol_helper::convert_domain_id_to_mongol(&value.device_id)?;
        let owner_id = mongol_helper::convert_domain_id_to_mongol(&value.owner.id)?;
        Ok(
            Self
            {
                value: value.value.clone(),
                device_id: device_id,
                owner_id: owner_id,
            }
        )
    }
}