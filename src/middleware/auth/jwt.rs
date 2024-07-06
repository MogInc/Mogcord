use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::model::{misc::ServerError, user::UserFlag};

const JWT_TTL_MINS: i64 = 10;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims
{
    pub sub: String,
    pub user_flag: UserFlag,
    pub exp: usize,
}

#[derive(PartialEq)]
pub enum TokenStatus
{
    AllowExpired,
    DisallowExpired,
}

pub struct CreateTokenRequest
{
    user_id: String,
    user_flag: UserFlag,
}

impl CreateTokenRequest
{
    pub fn new(user_id: String, user_flag: UserFlag) -> Self
    {
        Self
        {
            user_id: user_id,
            user_flag: user_flag,
        }
    }
}

pub fn create_token(request: &CreateTokenRequest) -> Result<String, ServerError>
{
    let claims = Claims
    {
        sub: request.user_id.clone(),
        user_flag: request.user_flag.clone(),
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

pub fn extract_token(token: &str, token_status: TokenStatus) -> Result<Claims, ServerError>
{
    let jwt_key = env::var("JWT_KEY")
        .map_err(|_| ServerError::JWTKeyNotSet)?;

    let mut validation = Validation::default();
    
    if token_status == TokenStatus::AllowExpired
    {
        validation.validate_exp = false;
    }
    
    match decode::<Claims>(token,&DecodingKey::from_secret(jwt_key.as_ref()), &validation)
    {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => 
        {
            match *err.kind()
            {
                ErrorKind::ExpiredSignature => Err(ServerError::JWTTokenExpired),
                _ => Err(ServerError::JWTTokenInvalid),
            }
        },
    }
}