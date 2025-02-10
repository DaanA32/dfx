mod fields;
pub(crate) mod limits;
pub mod types;

pub use fields::*;

use crate::field_map::Tag;

pub mod converters;

#[derive(Debug, Clone)]
pub enum ConversionError {
    EncodingError,
    IntParseErr,
    InvalidTag { tag: Tag, expected: Tag },
}
