use argon2::password_hash::rand_core::{OsRng, RngCore};
use base64::{alphabet, engine::{general_purpose, GeneralPurpose}, Engine};
use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::model::user::User;

use super::RefreshTokenFlag;

const REFRESH_TOKEN_TTL_IN_DAYS: i64 = 365;

#[derive(Deserialize)]
pub struct RefreshToken
{
    pub value: String,
    pub device_id: String,
    pub expiration_date: DateTime<Utc>,
    pub flag: RefreshTokenFlag,
    pub owner: User,
}

impl RefreshToken
{
    pub fn create_token(owner: User) -> Self
    {
        let mut random_number = [0u8; 64];
            
        let mut rng = OsRng;
        rng.fill_bytes(&mut random_number);
        
        const CUSTOM_ENGINE: GeneralPurpose = GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

        let refresh_token = CUSTOM_ENGINE.encode(&random_number);
        
        return Self
        {
            value: refresh_token,
            device_id: Uuid::now_v7().to_string(),
            expiration_date: (Utc::now() + Duration::days(REFRESH_TOKEN_TTL_IN_DAYS)),
            flag: RefreshTokenFlag::None,
            owner: owner,
        };
    }
}