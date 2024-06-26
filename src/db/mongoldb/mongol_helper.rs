use chrono::{DateTime, Datelike, NaiveDate, Utc};
use mongodb::bson;

pub trait MongolHelper
{
    fn convert_to_bson_datetime(&self) -> Result<bson::DateTime, bson::datetime::Error>;
}

impl MongolHelper for DateTime<Utc>
{
    fn convert_to_bson_datetime(&self) -> Result<bson::DateTime, bson::datetime::Error>
    {
        let date = self.date_naive();
        bson::DateTime::builder()
            .year(date.year())
            .month(date.month() as u8)
            .day(date.day() as u8)
            .build()
    }
}

impl MongolHelper for NaiveDate
{
    fn convert_to_bson_datetime(&self) -> Result<bson::DateTime, bson::datetime::Error>
    {
        bson::DateTime::builder()
            .year(self.year())
            .month(self.month() as u8)
            .day(self.day() as u8)
            .build()
    }
}