pub const MICROS_PER_MILLIS: u32 = 1000;
pub const NANOS_PER_MICRO: u32 = 1000;
pub const TICKS_PER_MICROSECOND: u32 = 10;
pub const NANOSECONDS_PER_TICK: u32 = 100;

// https://docs.rs/chrono/latest/chrono/format/strftime/index.html
pub const DATE_TIME_FORMAT_WITH_NANOSECONDS: &str = "%Y%m%d-%H:%M:%S.%f";
pub const DATE_TIME_FORMAT_WITH_MICROSECONDS: &str = "%Y%m%d-%H:%M:%S.%6f";
pub const DATE_TIME_FORMAT_WITH_MILLISECONDS: &str = "%Y%m%d-%H:%M:%S.%3f";
pub const DATE_TIME_FORMAT_WITHOUT_MILLISECONDS: &str = "%Y%m%d-%H:%M:%S";
pub const DATE_ONLY_FORMAT: &str = "%Y%m%d";
pub const TIME_ONLY_FORMAT_WITH_NANOSECONDS: &str = "%H:%M:%S.%f";
pub const TIME_ONLY_FORMAT_WITH_MICROSECONDS: &str = "%H:%M:%S.%6f";
pub const TIME_ONLY_FORMAT_WITH_MILLISECONDS: &str = "%H:%M:%S.%3f";
pub const TIME_ONLY_FORMAT_WITHOUT_MILLISECONDS: &str = "%H:%M:%S";

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum DateTimeFormat {
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
            DateTimeFormat::Seconds => DATE_TIME_FORMAT_WITH_MILLISECONDS,
        }
    }
    pub fn as_time_format(&self) -> &str {
        match self {
            DateTimeFormat::Nanoseconds => TIME_ONLY_FORMAT_WITH_NANOSECONDS,
            DateTimeFormat::Microseconds => TIME_ONLY_FORMAT_WITH_MICROSECONDS,
            DateTimeFormat::Milliseconds => TIME_ONLY_FORMAT_WITH_MILLISECONDS,
            DateTimeFormat::Seconds => TIME_ONLY_FORMAT_WITH_MILLISECONDS,
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

use chrono::format::parse;
use chrono::{DateTime, TimeZone, Utc};

use crate::field_map::FieldValue;
use crate::fields::converters::TryFrom;
use crate::fields::ConversionError;

impl<'a> TryFrom<&'a FieldValue> for DateTime<Utc> {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let time: &str = TryFrom::try_from(value)?;

        Utc.datetime_from_str(time, DATE_TIME_FORMAT_WITHOUT_MILLISECONDS)
            .map_err(|e| todo!())
    }
}

// impl<'a> TryFrom<&'a FieldValue> for &'a str {
//     type Error = ConversionError;

//     fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
//         std::str::from_utf8(value).map_err(|_| ConversionError::EncodingError)
//     }
// }
