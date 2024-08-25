use axum::async_trait;
use bson::doc;

use super::helper;
use crate::db::mongol::MongolDB;
use crate::model::channel::Channel;
use crate::model::{
    channel,
    error,
};
use crate::{
    bubble,
    server_error,
};

#[async_trait]
impl channel::Repository for MongolDB
{
    async fn get_channel<'input, 'err>(
        &'input self,
        channel_id: &'input str,
    ) -> error::Result<'err, Channel>
    {
        let channel_id_local =
            bubble!(helper::convert_domain_id_to_mongol(channel_id))?;

        let filter = doc! { "_id": channel_id_local };

        let user_option =
            self.channels().find_one(filter).await.map_err(|err| {
                server_error!(
                    error::Kind::Fetch,
                    error::OnType::Channel
                )
                .add_debug_info("error", err.to_string())
            })?;

        match user_option
        {
            Some(channel) => Ok(Channel::from(&channel)),
            None => Err(server_error!(
                error::Kind::NotFound,
                error::OnType::Channel
            )),
        }
    }
}
