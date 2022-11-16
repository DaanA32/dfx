mod fields;
pub(crate) mod limits;
use std::{
    borrow::{Borrow, Cow},
    ops::Deref,
};

pub(crate) use fields::*;

use crate::field_map::FieldValue;

pub(crate) mod converters;

#[derive(Debug, Clone)]
pub enum ConversionError {
    EncodingError,
}
