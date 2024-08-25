use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{
    PasswordHash,
    PasswordHasher,
    PasswordVerifier,
    SaltString,
};
use argon2::Argon2;
use tokio::task;

use super::error::{
    self,
    Kind,
    OnType,
};
use crate::server_error;

pub struct Hashing;

impl Hashing
{
    pub async fn hash_text<'err>(
        clear_text: &str
    ) -> error::Result<'err, String>
    {
        let clear_text = clear_text.to_string();

        let text_hashed = task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);

            let argon2 = Self::internal_give_argon_settings();

            //no need to return the salt, its stored inside the hash
            return argon2
                .hash_password(clear_text.as_bytes(), &salt)
                .map_err(|_| {
                    server_error!(
                        error::Kind::Create,
                        error::OnType::Hashing
                    )
                })
                .map(|hash| hash.to_string());
        })
        .await
        .map_err(|err| {
            server_error!(
                error::Kind::Unexpected,
                error::OnType::SpawnBlocking
            )
            .add_debug_info(
                "join error message",
                err.to_string(),
            )
        })??;

        Ok(text_hashed)
    }

    pub async fn verify_hash<'input, 'err>(
        clear_text: &'input str,
        hash: &'input str,
    ) -> error::Result<'err, ()>
    {
        let clear_text = clear_text.to_string();
        let hash = hash.to_string();

        task::spawn_blocking(move || {
            let parsed_hash = PasswordHash::new(&hash).map_err(|_| {
                server_error!(
                    Kind::Verifying,
                    OnType::Hashing
                )
            })?;

            let argon2 = Self::internal_give_argon_settings();

            argon2
                .verify_password(
                    clear_text.as_bytes(),
                    &parsed_hash,
                )
                .map_err(|_| {
                    server_error!(
                        Kind::Verifying,
                        OnType::Hashing
                    )
                })
        })
        .await
        .map_err(|_| {
            server_error!(
                Kind::Unexpected,
                OnType::SpawnBlocking
            )
        })??;

        Ok(())
    }

    fn internal_give_argon_settings<'schema>() -> Argon2<'schema>
    {
        Argon2::default()
    }
}
