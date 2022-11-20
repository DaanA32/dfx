mod fields;
pub(crate) mod limits;

pub use fields::*;

pub(crate) mod converters;

#[derive(Debug, Clone)]
pub enum ConversionError {
    EncodingError,
    IntParseErr,
}
