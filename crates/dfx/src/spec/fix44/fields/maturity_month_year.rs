use std::borrow::Cow;

use dfx_core::field_map::Tag;
use dfx_core::field_map::Field;
use dfx_core::fields::ConversionError;
use dfx_core::fields::converters::*;

/// MaturityMonthYear
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaturityMonthYear<'a> {
    inner: Cow<'a, Field>
}

impl<'a> MaturityMonthYear<'a> {
    pub fn new(value: &str) -> Self {
        let field = Field::new( MaturityMonthYear::tag(), value );
        Self {
            inner: Cow::Owned(field)
        }
    }
    pub const fn tag() -> Tag {
        200
    }
    pub fn value(&self) -> &str {
        // This will not panic due to the constraints on Field::new and the TryFrom impl
        self.inner.as_value().unwrap()
    }
}

impl<'a> TryFrom<&'a Field> for MaturityMonthYear<'a> {
    type Error = ConversionError;
    fn try_from(field: &'a Field) -> Result<Self, ConversionError> {
        if field.tag() != Self::tag() {
            return Err(ConversionError::InvalidTag { tag: field.tag(), expected: Self::tag() });
        }
        let _t: &str = field.as_value()?;
        Ok(Self { inner: Cow::Borrowed(field) })
    }
}
impl<'a> TryFrom<Field> for MaturityMonthYear<'a> {
    type Error = ConversionError;
    fn try_from(field: Field) -> Result<Self, ConversionError> {
        if field.tag() != Self::tag() {
            return Err(ConversionError::InvalidTag { tag: field.tag(), expected: Self::tag() });
        }
        let _t: &str = field.as_value()?;
        Ok(Self { inner: Cow::Owned(field) })
    }
}
impl<'a> Into<&'a Field> for &'a MaturityMonthYear<'a> {
    fn into(self) -> &'a Field {
        self.inner.as_ref()
    }
}
impl<'a> Into<Field> for &'a MaturityMonthYear<'a> {
    fn into(self) -> Field {
        self.inner.as_ref().clone()
    }
}
impl<'a> Into<Field> for MaturityMonthYear<'a> {
    fn into(self) -> Field {
        self.inner.into_owned()
    }
}
