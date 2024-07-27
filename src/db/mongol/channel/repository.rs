use axum::async_trait;
use bson::doc;

use crate::model::channel::Channel;
use crate::model::{channel, error};
use crate::db::mongol::MongolDB;
use super::helper;

#[async_trait]
impl channel::Repository for MongolDB
{
    async fn get_channel<'input, 'err>(&'input self, channel_id: &'input str) -> Result<Channel, error::Server<'err>>
    {
        let channel_id_local = helper::convert_domain_id_to_mongol(channel_id)?;

        let filter = doc! { "_id": channel_id_local };

        let user_option = self
            .channels()
            .find_one(filter)
            .await
            .map_err(|err| error::Server::new(
                error::Kind::Fetch,
                error::OnType::Channel,
                file!(),
                line!())
                .add_debug_info("error", err.to_string())
            )?;

        match user_option 
        {
            Some(channel) => Ok(Channel::from(&channel)),
            None => Err(error::Server::new(
                error::Kind::NotFound,
                error::OnType::Channel,
                file!(),
                line!(),
            )),
        }
    }
}

