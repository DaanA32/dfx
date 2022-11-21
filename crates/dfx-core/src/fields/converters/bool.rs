use crate::field_map::FieldValue;
use crate::fields::converters::TryFrom;
use crate::fields::ConversionError;

use super::IntoBytes;

impl<'a> TryFrom<&'a FieldValue> for bool {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        match ref_str {
            "Y" => Ok(true),
            "N" => Ok(false),
            _ => todo!(),
        }
    }
}

impl IntoBytes<FieldValue> for bool {
    fn as_bytes(&self) -> FieldValue {
        if *self {
            vec!['Y' as u8]
        } else {
            vec!['N' as u8]
        }
    }
}
