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

pub(crate) enum DateTimeFormat {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
}

use chrono::format::parse;
use chrono::{DateTime, Utc, TimeZone};

use crate::field_map::FieldValue;
use crate::fields::ConversionError;
use crate::fields::converters::TryFrom;

impl<'a> TryFrom<&'a FieldValue> for DateTime<Utc> {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let time: &str = TryFrom::try_from(value)?;

        Utc.datetime_from_str(time, DATE_TIME_FORMAT_WITHOUT_MILLISECONDS).map_err(|e| todo!())
    }
}


// impl<'a> TryFrom<&'a FieldValue> for &'a str {
//     type Error = ConversionError;

//     fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
//         std::str::from_utf8(value).map_err(|_| ConversionError::EncodingError)
//     }
// }
