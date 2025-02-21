use std::fmt;

use chrono::{
    DateTime,
    Datelike,
    NaiveDate,
    Timelike,
    Utc,
};
use mongodb::bson::{
    self,
    Uuid,
};
use serde::{
    Serialize,
    Serializer,
};

use crate::model::error;
use crate::server_error;

pub trait MongolHelper
{
    fn convert_to_bson_date(
        &self
    ) -> Result<bson::DateTime, bson::datetime::Error>;
    fn convert_to_bson_datetime(
        &self
    ) -> Result<bson::DateTime, bson::datetime::Error>;
}

impl MongolHelper for DateTime<Utc>
{
    fn convert_to_bson_date(
        &self
    ) -> Result<bson::DateTime, bson::datetime::Error>
    {
        let date = self.date_naive();
        MongolHelper::convert_to_bson_date(&date)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn convert_to_bson_datetime(
        &self
    ) -> Result<bson::DateTime, bson::datetime::Error>
    {
        bson::DateTime::builder()
            .year(self.year())
            .month(self.month() as u8)
            .day(self.day() as u8)
            .hour(self.hour() as u8)
            .minute(self.minute() as u8)
            .second(self.second() as u8)
            .build()
    }
}

impl MongolHelper for NaiveDate
{
    #[allow(clippy::cast_possible_truncation)]
    fn convert_to_bson_date(
        &self
    ) -> Result<bson::DateTime, bson::datetime::Error>
    {
        bson::DateTime::builder()
            .year(self.year())
            .month(self.month() as u8)
            .day(self.day() as u8)
            .build()
    }

    #[allow(clippy::cast_possible_truncation)]
    fn convert_to_bson_datetime(
        &self
    ) -> Result<bson::DateTime, bson::datetime::Error>
    {
        bson::DateTime::builder()
            .year(self.year())
            .month(self.month() as u8)
            .day(self.day() as u8)
            .build()
    }
}

pub fn convert_domain_id_to_mongol<'err>(id: &str)
    -> error::Result<'err, Uuid>
{
    Uuid::parse_str(id).map_err(|_| {
        server_error!(
            error::Kind::InValid,
            error::OnType::Mongo
        )
        .add_debug_info("incoming id", id.to_string())
    })
}

pub fn convert_domain_ids_to_mongol<'input, 'err>(
    ids: &'input [&'input str]
) -> error::Result<'err, Vec<Uuid>>
{
    ids.iter()
        .map(|id| convert_domain_id_to_mongol(id))
        .collect()
}

pub fn as_string<S, T>(
    v: &T,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: fmt::Display,
{
    let v = v.to_string();

    v.serialize(s)
}
