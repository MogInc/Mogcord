use std::env;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{model::{error::{self, Kind, OnType}, user}, server_error};

use super::ACCES_TOKEN_TTL_MIN;


#[derive(Serialize, Deserialize, Debug)]
pub struct Claims
{
    pub sub: String,
    pub user_flag: user::Flag,
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
    user_flag: &'user_info user::Flag,
}

impl<'user_info> CreateAccesTokenRequest<'user_info>
{
    #[must_use]
    pub fn new(user_id: &'user_info String, user_flag: &'user_info user::Flag) -> Self
    {
        Self
        {
            user_id,
            user_flag,
        }
    }
}

pub fn create_acces_token<'err>(request: &CreateAccesTokenRequest) -> Result<String, error::Server<'err>>
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
        .map_err(|_| server_error!(Kind::NotFound, OnType::AccesTokenHashKey))?;

    let acces_token = encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(acces_token_key.as_ref())
    ).map_err(|_| server_error!(Kind::Create, OnType::AccesToken))?;


    Ok(acces_token)
}

pub fn extract_acces_token<'err>(token: &str, acces_token_status: &TokenStatus) -> Result<Claims, error::Server<'err>>
{
    let acces_token_key = env::var("ACCES_TOKEN_KEY")
        .map_err(|_| server_error!(Kind::NotFound, OnType::AccesTokenHashKey))?;

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
                ErrorKind::ExpiredSignature => 
                {
                    Err(server_error!(Kind::Expired, OnType::AccesToken))
                },
                _ => 
                {
                    let err = server_error!(error::Kind::InValid, error::OnType::AccesToken)
                        .add_debug_info("acces token", token.to_string());

                    Err(err)
                },
            }
        },
    }
}