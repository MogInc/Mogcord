use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::model::{misc::ServerError, user::User};

pub const JWT_TTL_MINS: i64 = 10;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims
{
    sub: String,
    exp: usize,
}

impl Claims
{
    pub fn sub(self) -> String
    {
        return self.sub;
    }

    pub fn exp(self) -> usize
    {
        return self.exp;
    }
}

pub fn create_token(user: &User) -> Result<String, ServerError>
{
    let claims = Claims
    {
        sub: user.id.clone(),
        exp: (Utc::now() + Duration::minutes(JWT_TTL_MINS)).timestamp() as usize,
    };
    
    let jwt_key = env::var("JWT_KEY")
        .map_err(|_| ServerError::JWTKeyNotSet)?;

    let token = encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(jwt_key.as_ref())
    ).map_err(|_| ServerError::FailedCreatingToken)?;


    Ok(token)
}

pub fn extract_token(token: &str) -> Result<Claims, ServerError>
{
    let jwt_key = env::var("JWT_KEY")
        .map_err(|_| ServerError::JWTKeyNotSet)?;

    match decode::<Claims>(token,&DecodingKey::from_secret(jwt_key.as_ref()), &Validation::default())
    {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => {
            match *err.kind()
            {
                ErrorKind::ExpiredSignature => Err(ServerError::JWTTokenExpired),
                _ => Err(ServerError::JWTTokenInvalid)
            }
        },
    }
}