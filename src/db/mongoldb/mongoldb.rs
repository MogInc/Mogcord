use std::time::Duration;

use mongodb::{error::Error, options::{ClientOptions, Compressor}, results::InsertOneResult, Client, Collection};

use super::model::mongol_user::MongolUser;

#[derive(Clone, Debug)]
pub struct MongolDB
{
    users: Collection<MongolUser>
}

impl MongolDB
{
    pub async fn init(connection_string: &str) 
        -> Result<Self, Box<dyn std::error::Error>>
    {
        let mut client_options = ClientOptions::parse(connection_string).await?;

        client_options.app_name = Some("Mogcord".to_string());
        client_options.connect_timeout = Some(Duration::from_secs(30));
        client_options.compressors = Some(vec![
            Compressor::Snappy,
            Compressor::Zlib {
                level: Default::default(),
            },
            Compressor::Zstd {
                level: Default::default(),
            },
        ]);
    
        let client = Client::with_options(client_options)?;

        let db = client.database("db_mogcord");

        let users: Collection<MongolUser> = db.collection("users");

        Ok(Self { users : users })
    }

    pub async fn create_user(&self, user: MongolUser)
        -> Result<InsertOneResult, Error>
    {
        return self.users.insert_one(&user, None).await;
    }
}