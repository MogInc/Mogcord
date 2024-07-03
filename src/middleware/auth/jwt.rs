use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::model::misc::ServerError;

#[derive(Serialize, Deserialize, Debug)]
struct Claims
{
    sub: String,
    exp: usize,
}

pub fn create_token() -> Result<String, ServerError>
{
    let claims = Claims
    {
        sub: String::from("Appel"),
        exp: (Utc::now() + Duration::minutes(10)).timestamp() as usize,
    };
    
    let token = encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret("JWT_KEY".as_ref())
    ).map_err(|_| ServerError::FailedCreatingToken)?;


    Ok(token)
}

pub fn verify_token() -> Result<bool, ServerError>
{

}