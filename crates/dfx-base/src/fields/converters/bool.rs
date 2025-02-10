use crate::field_map::FieldValue;
use crate::fields::converters::TryFromFieldValue;
use crate::fields::ConversionError;

use super::IntoFieldValue;

impl<'a> TryFromFieldValue<&'a FieldValue> for bool {
    type Error = ConversionError;

    fn try_from_field_value(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFromFieldValue::try_from_field_value(value)?;

        match ref_str {
            "Y" => Ok(true),
            "N" => Ok(false),
            _ => todo!(),
        }
    }
}

impl IntoFieldValue<FieldValue> for bool {
    fn into_field_value(&self) -> FieldValue {
        if *self {
            vec![b'Y'].into()
        } else {
            vec![b'N'].into()
        }
    }
}
