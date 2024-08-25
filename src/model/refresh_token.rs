mod flag;
mod repository;

pub use flag::*;
pub use repository::*;

use argon2::password_hash::rand_core::{
    OsRng,
    RngCore,
};
use base64::engine::{
    general_purpose,
    GeneralPurpose,
};
use base64::{
    alphabet,
    Engine,
};
use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{
    DateTime,
    Duration,
    Utc,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::model::user::User;
use crate::server_error;

use super::error;

const REFRESH_TOKEN_TTL_IN_DAYS: i64 = 30;

#[derive(Deserialize)]
pub struct RefreshToken
{
    pub value: String,
    pub device_id: String,
    pub ip_addr: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub expiration_date: DateTime<Utc>,
    pub flag: Flag,
    pub owner: User,
}

impl RefreshToken
{
    #[must_use]
    pub fn create_token(
        owner: User,
        ip_addr: String,
        device_id_option: Option<String>,
    ) -> Self
    {
        const CUSTOM_ENGINE: GeneralPurpose = GeneralPurpose::new(
            &alphabet::URL_SAFE,
            general_purpose::NO_PAD,
        );

        let mut random_number = [0u8; 64];

        let mut rng = OsRng;
        rng.fill_bytes(&mut random_number);

        let refresh_token = CUSTOM_ENGINE.encode(random_number);

        Self {
            value: refresh_token,
            device_id: device_id_option.unwrap_or(Uuid::now_v7().to_string()),
            ip_addr,
            expiration_date: (Utc::now()
                + Duration::days(REFRESH_TOKEN_TTL_IN_DAYS)),
            flag: Flag::None,
            owner,
        }
    }

    pub fn refresh_expiration<'err>(mut self) -> error::Result<'err, Self>
    {
        if !self.internal_is_valid()
        {
            return Err(server_error!(
                error::Kind::NotAllowed,
                error::OnType::RefreshToken
            ));
        }

        self.expiration_date =
            Utc::now() + Duration::days(REFRESH_TOKEN_TTL_IN_DAYS);

        Ok(self)
    }

    fn internal_is_valid(&self) -> bool
    {
        matches!(self.flag, Flag::None)
    }
}
