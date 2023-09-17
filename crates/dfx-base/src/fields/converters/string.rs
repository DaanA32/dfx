use crate::field_map::FieldValue;
use crate::fields::converters::TryFrom;
use crate::fields::ConversionError;

use super::IntoFieldValue;

impl<'a> TryFrom<&'a FieldValue<'a>> for String {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        Ok(value.iter().map(|b| *b as char).collect())
    }
}

impl<'a> IntoFieldValue<'a, FieldValue<'a>> for String {
    fn into_field_value(&self) -> FieldValue<'a> {
        self.clone().into_bytes().into()
    }
}

impl<'a> TryFrom<&'a FieldValue<'a>> for &'a str {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        // TODO encoding latin1
        std::str::from_utf8(value).map_err(|_| ConversionError::EncodingError)
    }
}

impl<'a> IntoFieldValue<'a, FieldValue<'a>> for &&str {
    fn into_field_value(&self) -> FieldValue<'a> {
        let s: String = (**self).into();
        s.into_bytes().into()
    }
}

impl<'a> IntoFieldValue<'a, FieldValue<'a>> for &str {
    fn into_field_value(&'a self) -> FieldValue<'a> {
        self.as_bytes().into()
    }
}

impl<'a> IntoFieldValue<'a, FieldValue<'a>> for &&String {
    fn into_field_value(&self) -> FieldValue<'a> {
        let s: String = (**self).into();
        s.into_bytes().into()
    }
}

impl<'a> IntoFieldValue<'a, FieldValue<'a>> for &String {
    fn into_field_value(&self) -> FieldValue<'a> {
        let s: String = (*self).into();
        s.into_bytes().into()
    }
}


impl<'a> TryFrom<&'a FieldValue<'a>> for char {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            Err(ConversionError::EncodingError)
        } else {
            Ok(value[0] as char)
        }
    }
}


impl<'a> IntoFieldValue<'a, FieldValue<'a>> for char {
    fn into_field_value(&self) -> FieldValue<'a> {
        vec!(*self as u8).into()
    }
}
