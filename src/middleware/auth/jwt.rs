use std::env;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::model::{error::{self}, user};
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

pub fn create_acces_token<'stack>(request: &CreateAccesTokenRequest) -> Result<String, error::Server<'stack>>
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
        .map_err(|_| error::Server::new(
            error::Kind::NotFound,
            error::OnType::AccesTokenHashKey,
            file!(),
            line!(),
        ))?;

    let acces_token = encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(acces_token_key.as_ref())
    ).map_err(|_| error::Server::new(
        error::Kind::Create,
        error::OnType::AccesToken,
        file!(),
        line!(),
    ))?;


    Ok(acces_token)
}

pub fn extract_acces_token<'stack>(token: &str, acces_token_status: &TokenStatus) -> Result<Claims, error::Server<'stack>>
{
    let acces_token_key = env::var("ACCES_TOKEN_KEY")
        .map_err(|_| error::Server::new(
            error::Kind::NotFound,
            error::OnType::AccesTokenHashKey,
            file!(),
            line!(),
        ))?;

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
                    let err = error::Server::new(
                        error::Kind::Expired,
                        error::OnType::AccesToken,
                        file!(),
                        line!(),
                    );

                    Err(err)
                },
                _ => 
                {
                    let err = error::Server::new(
                        error::Kind::InValid,
                        error::OnType::AccesToken,
                        file!(),
                        line!())
                        .add_debug_info(token.to_string());

                    Err(err)
                },
            }
        },
    }
}