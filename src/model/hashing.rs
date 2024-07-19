use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use tokio::task;

use super::error;

pub struct Hashing;

impl Hashing
{
    pub async fn hash_text(clear_text: &str) -> Result<String, error::Server>
    {
        let clear_text = clear_text.to_string();

        let text_hashed = task::spawn_blocking(move || 
        {
            let salt = SaltString::generate(&mut OsRng);

            let argon2 = Self::internal_give_argon_settings();

            //no need to return the salt, its stored inside the hash
            return argon2.hash_password(clear_text.as_bytes(), &salt)
                .map_err(|_| error::Server::HashingPasswordFailed)
                .map(|hash| hash.to_string());
            
        }).await.map_err(|_| error::Server::HashingPasswordFailedBlocking)??;

        Ok(text_hashed)
    }

    pub async fn verify_hash(clear_text: &str, hash: &str) -> Result<(), error::Server>
    {
        let clear_text = clear_text.to_string();
        let hash = hash.to_string();

        task::spawn_blocking(move || 
        {
            let parsed_hash = PasswordHash::new(&hash)
                .map_err(|_| error::Server::VerifyingPasswordFailed)?;
            
            let argon2 = Self::internal_give_argon_settings();

            argon2.verify_password(clear_text.as_bytes(), &parsed_hash)
                  .map_err(|_| error::Server::VerifyingPasswordFailed)
        }).await.map_err(|_| error::Server::VerifyingPasswordFailedBlocking)??;

        Ok(())
    }

    fn internal_give_argon_settings<'schema>() -> Argon2<'schema>
    {
        Argon2::default()
    }
}