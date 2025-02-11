use crate::field_map::FieldValue;
use crate::fields::converters::TryFromFieldValue;
use crate::fields::ConversionError;

use super::IntoFieldValue;

impl<'a> TryFromFieldValue<&'a FieldValue> for usize {
    type Error = ConversionError;

    fn try_from_field_value(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFromFieldValue::try_from_field_value(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFromFieldValue<&'a FieldValue> for u128 {
    type Error = ConversionError;

    fn try_from_field_value(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFromFieldValue::try_from_field_value(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFromFieldValue<&'a FieldValue> for u64 {
    type Error = ConversionError;

    fn try_from_field_value(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFromFieldValue::try_from_field_value(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFromFieldValue<&'a FieldValue> for u32 {
    type Error = ConversionError;

    fn try_from_field_value(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let mut sum = 0;
        for byte in value.iter() {
            let byte = *byte;
            if byte.is_ascii_digit() {
                sum *= 10;
                sum += u32::from(byte) - u32::from(b'0');
            } else {
                return Err(ConversionError::IntParseErr);
            }
        }
        Ok(sum)
        // let ref_str: &str = TryFrom::try_from(value)?;

        // ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFromFieldValue<&'a FieldValue> for u16 {
    type Error = ConversionError;

    fn try_from_field_value(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFromFieldValue::try_from_field_value(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFromFieldValue<&'a FieldValue> for u8 {
    type Error = ConversionError;

    fn try_from_field_value(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFromFieldValue::try_from_field_value(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFromFieldValue<&'a FieldValue> for i128 {
    type Error = ConversionError;

    fn try_from_field_value(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFromFieldValue::try_from_field_value(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFromFieldValue<&'a FieldValue> for i64 {
    type Error = ConversionError;

    fn try_from_field_value(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFromFieldValue::try_from_field_value(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFromFieldValue<&'a FieldValue> for i32 {
    type Error = ConversionError;

    fn try_from_field_value(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFromFieldValue::try_from_field_value(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFromFieldValue<&'a FieldValue> for i16 {
    type Error = ConversionError;

    fn try_from_field_value(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFromFieldValue::try_from_field_value(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFromFieldValue<&'a FieldValue> for i8 {
    type Error = ConversionError;

    fn try_from_field_value(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFromFieldValue::try_from_field_value(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFromFieldValue<&'a FieldValue> for f64 {
    type Error = ConversionError;

    fn try_from_field_value(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFromFieldValue::try_from_field_value(value)?;
        //TODO replace with better function
        if let Some('+') = ref_str.chars().next() {
            return Err(ConversionError::IntParseErr);
        }
        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFromFieldValue<&'a FieldValue> for f32 {
    type Error = ConversionError;

    fn try_from_field_value(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFromFieldValue::try_from_field_value(value)?;
        //TODO replace with better function
        if let Some('+') = ref_str.chars().next() {
            return Err(ConversionError::IntParseErr);
        }
        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl IntoFieldValue<FieldValue> for usize {
    fn into_field_value(&self) -> FieldValue {
        format!("{self}").as_bytes().to_vec().into()
    }
}

impl IntoFieldValue<FieldValue> for f64 {
    fn into_field_value(&self) -> FieldValue {
        format!("{self}").as_bytes().to_vec().into()
    }
}

impl IntoFieldValue<FieldValue> for i64 {
    fn into_field_value(&self) -> FieldValue {
        format!("{self}").as_bytes().to_vec().into()
    }
}

impl IntoFieldValue<FieldValue> for i32 {
    fn into_field_value(&self) -> FieldValue {
        format!("{self}").as_bytes().to_vec().into()
    }
}
