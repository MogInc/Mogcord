use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::model::{error, user::UserFlag};

use super::ACCES_TOKEN_TTL_MIN;


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

pub struct CreateAccesTokenRequest<'user_info>
{
    user_id: &'user_info String,
    user_flag: &'user_info UserFlag,
}

impl<'user_info> CreateAccesTokenRequest<'user_info>
{
    #[must_use]
    pub fn new(user_id: &'user_info String, user_flag: &'user_info UserFlag) -> Self
    {
        Self
        {
            user_id,
            user_flag,
        }
    }
}

pub fn create_acces_token(request: &CreateAccesTokenRequest) -> Result<String, error::Server>
{
    let claims = Claims
    {
        sub: request.user_id.clone(),
        user_flag: request.user_flag.clone(),
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        exp: (Utc::now() + Duration::minutes(ACCES_TOKEN_TTL_MIN)).timestamp() as usize,
    };
    
    let acces_token_key = env::var("ACCES_TOKEN_KEY")
        .map_err(|_| error::Server::AccesTokenHashKeyNotSet)?;

    let acces_token = encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(acces_token_key.as_ref())
    ).map_err(|_| error::Server::FailedCreatingAccesToken)?;


    Ok(acces_token)
}

pub fn extract_acces_token(token: &str, acces_token_status: &TokenStatus) -> Result<Claims, error::Server>
{
    let acces_token_key = env::var("ACCES_TOKEN_KEY")
        .map_err(|_| error::Server::AccesTokenHashKeyNotSet)?;

    let mut validation = Validation::default();
    
    if acces_token_status == &TokenStatus::AllowExpired
    {
        validation.validate_exp = false;
    }
    
    match decode::<Claims>(token,&DecodingKey::from_secret(acces_token_key.as_ref()), &validation)
    {
        Ok(acces_token_data) => Ok(acces_token_data.claims),
        Err(err) => 
        {
            match *err.kind()
            {
                ErrorKind::ExpiredSignature => Err(error::Server::AccesTokenExpired),
                _ => Err(error::Server::AccesTokenInvalid),
            }
        },
    }
}