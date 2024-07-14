
use axum::async_trait;
use crate::{db::mongoldb::MongolDB, model::relation::RelationRepository};

#[async_trait]
impl RelationRepository for MongolDB
{
    
}