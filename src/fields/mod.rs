mod fields;
pub mod limits;
use std::{ops::Deref, borrow::{Cow, Borrow}};

pub use fields::*;

use crate::field_map::FieldValue;

pub mod converters;

#[derive(Debug, Clone)]
pub enum ConversionError {
    EncodingError,
}
