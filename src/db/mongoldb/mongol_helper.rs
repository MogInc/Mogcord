use chrono::{DateTime, Datelike, NaiveDate, Timelike, Utc};
use mongodb::bson::{self, Uuid};


use super::MongolError;

pub trait MongolHelper
{
    fn convert_to_bson_date(&self) -> Result<bson::DateTime, bson::datetime::Error>;
    fn convert_to_bson_datetime(&self) -> Result<bson::DateTime, bson::datetime::Error>;
}

impl MongolHelper for DateTime<Utc>
{
    fn convert_to_bson_date(&self) -> Result<bson::DateTime, bson::datetime::Error>
    {
        let date = self.date_naive();
        MongolHelper::convert_to_bson_date(&date)
    }

    fn convert_to_bson_datetime(&self) -> Result<bson::DateTime, bson::datetime::Error>
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
    fn convert_to_bson_date(&self) -> Result<bson::DateTime, bson::datetime::Error>
    {
        bson::DateTime::builder()
            .year(self.year())
            .month(self.month() as u8)
            .day(self.day() as u8)
            .build()
    }

    fn convert_to_bson_datetime(&self) -> Result<bson::DateTime, bson::datetime::Error>
    {
        bson::DateTime::builder()
            .year(self.year())
            .month(self.month() as u8)
            .day(self.day() as u8)
            .build()
    }
}


pub trait FromWithoutMetaInfo<T>
{
    fn from_without_meta_info(flag: T) -> Self;
}

pub fn convert_domain_id_to_mongol(id: &str)
 -> Result<Uuid, MongolError>
{
    Uuid::parse_str(id).map_err(|_| MongolError::InvalidID{ id: id.to_string() })
}