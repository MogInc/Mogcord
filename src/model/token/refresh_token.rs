use argon2::password_hash::rand_core::{OsRng, RngCore};
use base64::{alphabet, engine::{general_purpose, GeneralPurpose}, Engine};

pub struct RefreshToken
{
    value: String,
}

impl RefreshToken
{
    pub fn new(token: String) -> Self
    {
        Self
        {
            value: token,
        }
    }
}

impl RefreshToken
{
    pub fn value(&self) -> String
    {
        return self.value.to_string();
    }
}

impl RefreshToken
{
    pub fn create_token() -> Self
    {
        let mut random_number = [0u8; 64];
            
        let mut rng = OsRng;
        rng.fill_bytes(&mut random_number);
        
        const CUSTOM_ENGINE: GeneralPurpose = GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

        let refresh_token = CUSTOM_ENGINE.encode(&random_number);
        
        return Self
        {
            value: refresh_token,
        };
    }
}