mod repository;


use bson::{Bson, DateTime, Uuid};
use serde::{Deserialize, Serialize};

use crate::model::{error, refresh_token::{self, RefreshToken}};
use super::helper::{self, as_string, MongolHelper};


#[derive(Debug, Serialize, Deserialize)]
pub struct MongolRefreshToken
{
    pub value: String,
    pub device_id: Uuid,
    pub expiration_date: DateTime,
    #[serde(serialize_with = "as_string")]
    pub flag: refresh_token::Flag,
    pub owner_id: Uuid,
}

impl TryFrom<&RefreshToken> for MongolRefreshToken
{
    type Error = error::Server<'static>;

    fn try_from(value: &RefreshToken) -> Result<Self, Self::Error> 
    {
        let device_id = helper::convert_domain_id_to_mongol(&value.device_id)?;
        let owner_id = helper::convert_domain_id_to_mongol(&value.owner.id)?;

        let expiration_date = value
            .expiration_date
            .convert_to_bson_datetime()
            .map_err(|_| error::Server::new(
                error::Kind::InValid,
                error::OnType::Date,
                file!(),
                line!())
                .add_debug_info(value.expiration_date.to_rfc3339())
            )?;

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

impl From<refresh_token::Flag> for Bson 
{
    fn from(token_flag: refresh_token::Flag) -> Bson 
    {
        Bson::String(token_flag.to_string())
    }
}