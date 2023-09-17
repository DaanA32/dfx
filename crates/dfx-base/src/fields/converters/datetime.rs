#![allow(dead_code)]
#![allow(unused)]
pub const MICROS_PER_MILLIS: u32 = 1000;
pub const NANOS_PER_MICRO: u32 = 1000;
pub const TICKS_PER_MICROSECOND: u32 = 10;
pub const NANOSECONDS_PER_TICK: u32 = 100;

// https://docs.rs/chrono/latest/chrono/format/strftime/index.html
pub const DATE_TIME_FORMAT_WITH_NANOSECONDS_LEN: usize = 8 + 1 + 6 + 2 + 1 + 9;
pub const DATE_TIME_FORMAT_WITH_NANOSECONDS: &str = "%Y%m%d-%H:%M:%S.%f";
pub const DATE_TIME_FORMAT_WITH_MICROSECONDS_LEN: usize = 8 + 1 + 6 + 2 + 1 + 6;
pub const DATE_TIME_FORMAT_WITH_MICROSECONDS: &str = "%Y%m%d-%H:%M:%S.%6f";
pub const DATE_TIME_FORMAT_WITH_MILLISECONDS_LEN: usize = 8 + 1 + 6 + 2 + 1 + 3;
pub const DATE_TIME_FORMAT_WITH_MILLISECONDS: &str = "%Y%m%d-%H:%M:%S.%3f";
pub const DATE_TIME_FORMAT_WITHOUT_MILLISECONDS_LEN: usize = 8 + 1 + 6 + 2;
pub const DATE_TIME_FORMAT_WITHOUT_MILLISECONDS: &str = "%Y%m%d-%H:%M:%S";
pub const DATE_ONLY_FORMAT: &str = "%Y%m%d";
pub const TIME_ONLY_FORMAT_WITH_NANOSECONDS: &str = "%H:%M:%S.%f";
pub const TIME_ONLY_FORMAT_WITH_MICROSECONDS: &str = "%H:%M:%S.%6f";
pub const TIME_ONLY_FORMAT_WITH_MILLISECONDS: &str = "%H:%M:%S.%3f";
pub const TIME_ONLY_FORMAT_WITHOUT_MILLISECONDS: &str = "%H:%M:%S";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DateTimeFormat {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
}

impl DateTimeFormat {
    pub fn as_datetime_format(&self) -> &str {
        match self {
            DateTimeFormat::Nanoseconds => DATE_TIME_FORMAT_WITH_NANOSECONDS,
            DateTimeFormat::Microseconds => DATE_TIME_FORMAT_WITH_MICROSECONDS,
            DateTimeFormat::Milliseconds => DATE_TIME_FORMAT_WITH_MILLISECONDS,
            DateTimeFormat::Seconds => DATE_TIME_FORMAT_WITHOUT_MILLISECONDS,
        }
    }
    pub fn as_time_format(&self) -> &str {
        match self {
            DateTimeFormat::Nanoseconds => TIME_ONLY_FORMAT_WITH_NANOSECONDS,
            DateTimeFormat::Microseconds => TIME_ONLY_FORMAT_WITH_MICROSECONDS,
            DateTimeFormat::Milliseconds => TIME_ONLY_FORMAT_WITH_MILLISECONDS,
            DateTimeFormat::Seconds => TIME_ONLY_FORMAT_WITHOUT_MILLISECONDS,
        }
    }
}

impl std::convert::TryFrom<String> for DateTimeFormat {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Nanoseconds" => Ok(Self::Nanoseconds),
            "Microseconds" => Ok(Self::Microseconds),
            "Milliseconds" => Ok(Self::Milliseconds),
            "Seconds" => Ok(Self::Seconds),
            _ => Err("Valid format is ... TODO")
        }
    }
}

use chrono::{DateTime as ChronoDateTime, TimeZone, Utc, NaiveDateTime, NaiveDate, NaiveTime, Local};

use crate::field_map::FieldValue;
use crate::fields::converters::TryFrom;
use crate::fields::ConversionError;

use super::IntoFieldValue;

impl<'a> TryFrom<&'a FieldValue<'a>> for ChronoDateTime<Utc> {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        let time: &str = TryFrom::try_from(value)?;
        match Utc.datetime_from_str(time, DATE_TIME_FORMAT_WITHOUT_MILLISECONDS) {
            Ok(t) => Ok(t),
            Err(e) => match Utc.datetime_from_str(time, DATE_TIME_FORMAT_WITH_MILLISECONDS) {
                Ok(t) => Ok(t),
                Err(_) => todo!(),
            },
        }
    }
}

pub type Date = NaiveDate;
pub type Time = NaiveTime;
pub type DateTime = NaiveDateTime;

impl<'a> TryFrom<&'a FieldValue<'a>> for DateTime {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        let time: &str = TryFrom::try_from(value)?;
        let format = match time.len() {
            DATE_TIME_FORMAT_WITH_NANOSECONDS_LEN => Ok(DATE_TIME_FORMAT_WITH_NANOSECONDS),
            DATE_TIME_FORMAT_WITH_MICROSECONDS_LEN => Ok(DATE_TIME_FORMAT_WITH_MICROSECONDS),
            DATE_TIME_FORMAT_WITH_MILLISECONDS_LEN => Ok(DATE_TIME_FORMAT_WITH_MILLISECONDS),
            DATE_TIME_FORMAT_WITHOUT_MILLISECONDS_LEN => Ok(DATE_TIME_FORMAT_WITHOUT_MILLISECONDS),
            len => Err(ConversionError::EncodingError),
        }?;
        match NaiveDateTime::parse_from_str(time, format) {
            Ok(v) => Ok(v),
            Err(e) => todo!("{e}"),
        }
    }
}

impl<'a> TryFrom<&'a FieldValue<'a>> for Time {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl<'a> TryFrom<&'a FieldValue<'a>> for Date {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl<'a> IntoFieldValue<'a, FieldValue<'a>> for DateTime {
    fn into_field_value(&self) -> FieldValue<'a> {
        todo!()
    }
}

impl<'a> IntoFieldValue<'a, FieldValue<'a>> for Date {
    fn into_field_value(&self) -> FieldValue<'a> {
        todo!()
    }
}

impl<'a> IntoFieldValue<'a, FieldValue<'a>> for Time {
    fn into_field_value(&self) -> FieldValue<'a> {
        todo!()
    }
}
