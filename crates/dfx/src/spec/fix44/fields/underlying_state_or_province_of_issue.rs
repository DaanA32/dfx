use std::borrow::Cow;

use dfx_core::field_map::Tag;
use dfx_core::field_map::Field;
use dfx_core::fields::ConversionError;
use dfx_core::fields::converters::*;

/// UnderlyingStateOrProvinceOfIssue
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnderlyingStateOrProvinceOfIssue<'a> {
    inner: Cow<'a, Field>
}

impl<'a> UnderlyingStateOrProvinceOfIssue<'a> {
    pub fn new(value: &str) -> Self {
        let field = Field::new( UnderlyingStateOrProvinceOfIssue::tag(), value );
        Self {
            inner: Cow::Owned(field)
        }
    }
    pub const fn tag() -> Tag {
        593
    }
    pub fn value(&self) -> &str {
        // This will not panic due to the constraints on Field::new and the TryFrom impl
        self.inner.as_value().unwrap()
    }
}

impl<'a> TryFrom<&'a Field> for UnderlyingStateOrProvinceOfIssue<'a> {
    type Error = ConversionError;
    fn try_from(field: &'a Field) -> Result<Self, ConversionError> {
        if field.tag() != Self::tag() {
            return Err(ConversionError::InvalidTag { tag: field.tag(), expected: Self::tag() });
        }
        let _t: &str = field.as_value()?;
        Ok(Self { inner: Cow::Borrowed(field) })
    }
}
impl<'a> TryFrom<Field> for UnderlyingStateOrProvinceOfIssue<'a> {
    type Error = ConversionError;
    fn try_from(field: Field) -> Result<Self, ConversionError> {
        if field.tag() != Self::tag() {
            return Err(ConversionError::InvalidTag { tag: field.tag(), expected: Self::tag() });
        }
        let _t: &str = field.as_value()?;
        Ok(Self { inner: Cow::Owned(field) })
    }
}
impl<'a> Into<&'a Field> for &'a UnderlyingStateOrProvinceOfIssue<'a> {
    fn into(self) -> &'a Field {
        self.inner.as_ref()
    }
}
impl<'a> Into<Field> for &'a UnderlyingStateOrProvinceOfIssue<'a> {
    fn into(self) -> Field {
        self.inner.as_ref().clone()
    }
}
impl<'a> Into<Field> for UnderlyingStateOrProvinceOfIssue<'a> {
    fn into(self) -> Field {
        self.inner.into_owned()
    }
}
