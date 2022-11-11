use crate::field_map::FieldValue;
use crate::fields::ConversionError;
use crate::fields::converters::TryFrom;

impl<'a> TryFrom<&'a FieldValue> for String {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        Ok(value.iter().map(|b| *b as char).collect())
    }
}


impl<'a> TryFrom<&'a FieldValue> for &'a str {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        std::str::from_utf8(value).map_err(|_| ConversionError::EncodingError)
    }
}
