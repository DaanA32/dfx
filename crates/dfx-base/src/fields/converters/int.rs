use crate::field_map::FieldValue;
use crate::fields::converters::TryFrom;
use crate::fields::ConversionError;

use super::IntoBytes;

impl<'a> TryFrom<&'a FieldValue> for usize {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue> for u128 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue> for u64 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue> for u32 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
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

impl<'a> TryFrom<&'a FieldValue> for u16 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue> for u8 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue> for i128 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue> for i64 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue> for i32 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue> for i16 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue> for i8 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue> for f64 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;
        //TODO replace with better function
        if let Some('+') = ref_str.chars().next() {
            return Err(ConversionError::IntParseErr);
        }
        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue> for f32 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;
        //TODO replace with better function
        if let Some('+') = ref_str.chars().next() {
            return Err(ConversionError::IntParseErr);
        }
        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl IntoBytes<FieldValue> for usize {
    fn as_bytes(&self) -> FieldValue {
        format!("{self}").as_bytes().to_vec().into()
    }
}

impl IntoBytes<FieldValue> for f64 {
    fn as_bytes(&self) -> FieldValue {
        format!("{self}").as_bytes().to_vec().into()
    }
}

impl IntoBytes<FieldValue> for i64 {
    fn as_bytes(&self) -> FieldValue {
        format!("{self}").as_bytes().to_vec().into()
    }
}
