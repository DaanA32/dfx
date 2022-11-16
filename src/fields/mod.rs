mod fields;
pub(crate) mod limits;
use std::{ops::Deref, borrow::{Cow, Borrow}};

pub(crate) use fields::*;

use crate::field_map::FieldValue;

pub(crate) mod converters;

#[derive(Debug, Clone)]
pub enum ConversionError {
    EncodingError,
}
