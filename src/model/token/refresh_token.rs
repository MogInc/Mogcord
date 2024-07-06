use argon2::password_hash::rand_core::{OsRng, RngCore};
use base64::{alphabet, engine::{general_purpose, GeneralPurpose}, Engine};
use serde::Deserialize;
use uuid::Uuid;

use crate::model::user::User;

#[derive(Deserialize)]
pub struct RefreshToken
{
    //TODO: refresh token flag
    pub value: String,
    pub device_id: String,
    pub owner: User,
}

impl RefreshToken
{
    pub fn new(token: String, device_id: String, owner: User) -> Self
    {
        Self
        {
            value: token,
            device_id: device_id,
            owner: owner,
        }
    }
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
            owner: owner,
        };
    }
}