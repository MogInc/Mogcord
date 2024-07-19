use bson::{Bson, DateTime, Uuid};
use serde::{Deserialize, Serialize};

use crate::{db::mongol::{as_string, helper, MongolHelper}, model::{error, refresh_token::{RefreshToken, RefreshTokenFlag}}};


#[derive(Debug, Serialize, Deserialize)]
pub struct MongolRefreshToken
{
    pub value: String,
    pub device_id: Uuid,
    pub expiration_date: DateTime,
    #[serde(serialize_with = "as_string")]
    pub flag: RefreshTokenFlag,
    pub owner_id: Uuid,
}

impl TryFrom<&RefreshToken> for MongolRefreshToken
{
    type Error = error::Server;

    fn try_from(value: &RefreshToken) -> Result<Self, Self::Error> 
    {
        let device_id = helper::convert_domain_id_to_mongol(&value.device_id)?;
        let owner_id = helper::convert_domain_id_to_mongol(&value.owner.id)?;

        let expiration_date = value
            .expiration_date
            .convert_to_bson_datetime()
            .map_err(|_| error::Server::FailedDateParsing)?;

        Ok(
            Self
            {
                value: value.value.clone(),
                device_id,
                expiration_date,
                flag: value.flag.clone(),
                owner_id,
            }
        )
    }
}

impl From<RefreshTokenFlag> for Bson 
{
    fn from(token_flag: RefreshTokenFlag) -> Bson 
    {
        Bson::String(token_flag.to_string())
    }
}