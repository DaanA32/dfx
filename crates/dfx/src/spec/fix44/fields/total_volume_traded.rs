use std::borrow::Cow;

use dfx_core::field_map::Tag;
use dfx_core::field_map::Field;
use dfx_core::fields::ConversionError;
#[allow(unused)]
use dfx_core::fields::converters::*;

/// TotalVolumeTraded
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TotalVolumeTraded<'a> {
    inner: Cow<'a, Field>
}

impl<'a> TotalVolumeTraded<'a> {
    pub fn new(value: Decimal) -> Self {
        let field = Field::new( TotalVolumeTraded::tag(), value );
        Self {
            inner: Cow::Owned(field)
        }
    }
    pub const fn tag() -> Tag {
        387
    }
    pub fn value(&self) -> Decimal {
        // This will not panic due to the constraints on Field::new and the TryFrom impl
        self.inner.as_value().unwrap()
    }
}

impl<'a> std::convert::TryFrom<&'a Field> for TotalVolumeTraded<'a> {
    type Error = ConversionError;
    fn try_from(field: &'a Field) -> Result<Self, ConversionError> {
        if field.tag() != Self::tag() {
            return Err(ConversionError::InvalidTag { tag: field.tag(), expected: Self::tag() });
        }
        let _t: Decimal = field.as_value()?;
        Ok(Self { inner: Cow::Borrowed(field) })
    }
}
impl<'a> std::convert::TryFrom<Field> for TotalVolumeTraded<'a> {
    type Error = ConversionError;
    fn try_from(field: Field) -> Result<Self, ConversionError> {
        if field.tag() != Self::tag() {
            return Err(ConversionError::InvalidTag { tag: field.tag(), expected: Self::tag() });
        }
        let _t: Decimal = field.as_value()?;
        Ok(Self { inner: Cow::Owned(field) })
    }
}
impl<'a> Into<&'a Field> for &'a TotalVolumeTraded<'a> {
    fn into(self) -> &'a Field {
        self.inner.as_ref()
    }
}
impl<'a> Into<Field> for &'a TotalVolumeTraded<'a> {
    fn into(self) -> Field {
        self.inner.as_ref().clone()
    }
}
impl<'a> Into<Field> for TotalVolumeTraded<'a> {
    fn into(self) -> Field {
        self.inner.into_owned()
    }
}
