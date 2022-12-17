use std::borrow::Cow;

use dfx_core::field_map::Tag;
use dfx_core::field_map::Field;
use dfx_core::fields::ConversionError;
#[allow(unused)]
use dfx_core::fields::converters::*;

/// Nested2PartySubIdType
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nested2PartySubIdType<'a> {
    inner: Cow<'a, Field>
}

impl<'a> Nested2PartySubIdType<'a> {
    pub fn new(value: i64) -> Self {
        let field = Field::new( Nested2PartySubIdType::tag(), value );
        Self {
            inner: Cow::Owned(field)
        }
    }
    pub const fn tag() -> Tag {
        807
    }
    pub fn value(&self) -> i64 {
        // This will not panic due to the constraints on Field::new and the TryFrom impl
        self.inner.as_value().unwrap()
    }
}

impl<'a> std::convert::TryFrom<&'a Field> for Nested2PartySubIdType<'a> {
    type Error = ConversionError;
    fn try_from(field: &'a Field) -> Result<Self, ConversionError> {
        if field.tag() != Self::tag() {
            return Err(ConversionError::InvalidTag { tag: field.tag(), expected: Self::tag() });
        }
        let _t: i64 = field.as_value()?;
        Ok(Self { inner: Cow::Borrowed(field) })
    }
}
impl<'a> std::convert::TryFrom<Field> for Nested2PartySubIdType<'a> {
    type Error = ConversionError;
    fn try_from(field: Field) -> Result<Self, ConversionError> {
        if field.tag() != Self::tag() {
            return Err(ConversionError::InvalidTag { tag: field.tag(), expected: Self::tag() });
        }
        let _t: i64 = field.as_value()?;
        Ok(Self { inner: Cow::Owned(field) })
    }
}
impl<'a> Into<&'a Field> for &'a Nested2PartySubIdType<'a> {
    fn into(self) -> &'a Field {
        self.inner.as_ref()
    }
}
impl<'a> Into<Field> for &'a Nested2PartySubIdType<'a> {
    fn into(self) -> Field {
        self.inner.as_ref().clone()
    }
}
impl<'a> Into<Field> for Nested2PartySubIdType<'a> {
    fn into(self) -> Field {
        self.inner.into_owned()
    }
}
