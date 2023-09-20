use crate::field_map::FieldValue;
use crate::fields::converters::TryFromFieldValue;
use crate::fields::ConversionError;

use super::IntoFieldValue;

impl<'a> TryFromFieldValue<&'a FieldValue> for String {
    type Error = ConversionError;

    fn try_from_field_value(value: &'a FieldValue) -> Result<Self, Self::Error> {
        Ok(value.iter().map(|b| *b as char).collect())
    }
}

impl IntoFieldValue<FieldValue> for String {
    fn into_field_value(&self) -> FieldValue {
        self.clone().into_bytes().into()
    }
}

impl<'a> TryFromFieldValue<&'a FieldValue> for &'a str {
    type Error = ConversionError;

    fn try_from_field_value(value: &'a FieldValue) -> Result<Self, Self::Error> {
        // TODO encoding latin1
        std::str::from_utf8(value).map_err(|_| ConversionError::EncodingError)
    }
}

impl IntoFieldValue<FieldValue> for &&str {
    fn into_field_value(&self) -> FieldValue {
        let s: String = (**self).into();
        s.into_bytes().into()
    }
}

impl IntoFieldValue<FieldValue> for &str {
    fn into_field_value(&self) -> FieldValue {
        let s: String = (*self).into();
        s.into_bytes().into()
    }
}

impl IntoFieldValue<FieldValue> for &&String {
    fn into_field_value(&self) -> FieldValue {
        let s: String = (**self).into();
        s.into_bytes().into()
    }
}

impl IntoFieldValue<FieldValue> for &String {
    fn into_field_value(&self) -> FieldValue {
        let s: String = (*self).into();
        s.into_bytes().into()
    }
}


impl<'a> TryFromFieldValue<&'a FieldValue> for char {
    type Error = ConversionError;

    fn try_from_field_value(value: &'a FieldValue) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            Err(ConversionError::EncodingError)
        } else {
            Ok(value[0] as char)
        }
    }
}


impl IntoFieldValue<FieldValue> for char {
    fn into_field_value(&self) -> FieldValue {
        vec!(*self as u8).into()
    }
}
