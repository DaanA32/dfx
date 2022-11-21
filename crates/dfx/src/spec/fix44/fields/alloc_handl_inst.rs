use std::borrow::Cow;

use dfx_core::field_map::Tag;
use dfx_core::field_map::Field;
use dfx_core::fields::ConversionError;
use dfx_core::fields::converters::*;

/// AllocHandlInst
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AllocHandlInst<'a> {
    inner: Cow<'a, Field>
}

impl<'a> AllocHandlInst<'a> {
    pub fn new(value: i64) -> Self {
        let field = Field::new( AllocHandlInst::tag(), value );
        Self {
            inner: Cow::Owned(field)
        }
    }
    pub const fn tag() -> Tag {
        209
    }
    pub fn value(&self) -> i64 {
        // This will not panic due to the constraints on Field::new and the TryFrom impl
        self.inner.as_value().unwrap()
    }
}

impl<'a> TryFrom<&'a Field> for AllocHandlInst<'a> {
    type Error = ConversionError;
    fn try_from(field: &'a Field) -> Result<Self, ConversionError> {
        if field.tag() != Self::tag() {
            return Err(ConversionError::InvalidTag { tag: field.tag(), expected: Self::tag() });
        }
        let _t: i64 = field.as_value()?;
        Ok(Self { inner: Cow::Borrowed(field) })
    }
}
impl<'a> TryFrom<Field> for AllocHandlInst<'a> {
    type Error = ConversionError;
    fn try_from(field: Field) -> Result<Self, ConversionError> {
        if field.tag() != Self::tag() {
            return Err(ConversionError::InvalidTag { tag: field.tag(), expected: Self::tag() });
        }
        let _t: i64 = field.as_value()?;
        Ok(Self { inner: Cow::Owned(field) })
    }
}
impl<'a> Into<&'a Field> for &'a AllocHandlInst<'a> {
    fn into(self) -> &'a Field {
        self.inner.as_ref()
    }
}
impl<'a> Into<Field> for &'a AllocHandlInst<'a> {
    fn into(self) -> Field {
        self.inner.as_ref().clone()
    }
}
impl<'a> Into<Field> for AllocHandlInst<'a> {
    fn into(self) -> Field {
        self.inner.into_owned()
    }
}
