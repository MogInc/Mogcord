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
    pub async fn hash_text<'stack>(clear_text: &str) -> Result<String, error::Server<'stack>>
    {
        let clear_text = clear_text.to_string();

        let text_hashed = task::spawn_blocking(move || 
        {
            let salt = SaltString::generate(&mut OsRng);

            let argon2 = Self::internal_give_argon_settings();

            //no need to return the salt, its stored inside the hash
            return argon2.hash_password(clear_text.as_bytes(), &salt)
                .map_err(|_| error::Server::new(
                    error::Kind::Create,
                    error::OnType::Hashing,
                    file!(),
                    line!()
                ))
                .map(|hash| hash.to_string());
            
        }).await.map_err(|err| error::Server::new(
            error::Kind::Unexpected,
            error::OnType::SpawnBlocking,
            file!(),
            line!())
            .add_debug_info(err.to_string())
        )??;

        Ok(text_hashed)
    }

    pub async fn verify_hash<'input, 'stack>(clear_text: &'input str, hash: &'input str) -> Result<(), error::Server<'stack>>
    {
        let clear_text = clear_text.to_string();
        let hash = hash.to_string();

        task::spawn_blocking(move || 
        {
            let parsed_hash = PasswordHash::new(&hash)
                .map_err(|_| error::Server::new(
                    error::Kind::Verifying,
                    error::OnType::Hashing,
                    file!(),
                    line!(),
                ))?;
            
            let argon2 = Self::internal_give_argon_settings();

            argon2.verify_password(clear_text.as_bytes(), &parsed_hash)
                  .map_err(|_| error::Server::new(
                    error::Kind::Verifying,
                    error::OnType::Hashing,
                    file!(),
                    line!(),
                    ))
        }).await.map_err(|_| error::Server::new(
            error::Kind::Unexpected,
            error::OnType::SpawnBlocking,
            file!(),
            line!(),
        ))??;

        Ok(())
    }

    fn internal_give_argon_settings<'schema>() -> Argon2<'schema>
    {
        Argon2::default()
    }
}