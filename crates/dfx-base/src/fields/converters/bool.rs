use crate::field_map::FieldValue;
use crate::fields::converters::TryFrom;
use crate::fields::ConversionError;

use super::IntoFieldValue;

impl<'a> TryFrom<&'a FieldValue<'a>> for bool {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        match ref_str {
            "Y" => Ok(true),
            "N" => Ok(false),
            _ => todo!(),
        }
    }
}

impl<'a> IntoFieldValue<'a, FieldValue<'a>> for bool {
    fn into_field_value(&self) -> FieldValue<'a> {
        if *self {
            vec!['Y' as u8].into()
        } else {
            vec!['N' as u8].into()
        }
    }
}
