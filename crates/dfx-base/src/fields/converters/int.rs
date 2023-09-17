use crate::field_map::FieldValue;
use crate::fields::converters::TryFrom;
use crate::fields::ConversionError;

use super::IntoFieldValue;

impl<'a> TryFrom<&'a FieldValue<'a>> for usize {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue<'a>> for u128 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue<'a>> for u64 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue<'a>> for u32 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        let mut sum = 0;
        for byte in value.iter() {
            let byte = *byte;
            if byte >= b'0' && byte <= b'9' {
                sum = 10 * sum;
                sum += byte as u32 - b'0' as u32;
            } else {
                return Err(ConversionError::IntParseErr);
            }
        }
        Ok(sum)
        // let ref_str: &str = TryFrom::try_from(value)?;

        // ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue<'a>> for u16 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue<'a>> for u8 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue<'a>> for i128 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue<'a>> for i64 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue<'a>> for i32 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue<'a>> for i16 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue<'a>> for i8 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue<'a>> for f64 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;
        //TODO replace with better function
        match ref_str.chars().next() {
            Some('+') => {
                return Err(ConversionError::IntParseErr);
            },
            _ => {}
        }
        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue<'a>> for f32 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue<'a>) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;
        //TODO replace with better function
        match ref_str.chars().next() {
            Some('+') => {
                return Err(ConversionError::IntParseErr);
            },
            _ => {}
        }
        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> IntoFieldValue<'a, FieldValue<'a>> for usize {
    fn into_field_value(self) -> FieldValue<'a> {
        format!("{}", self).as_bytes().to_vec().into()
    }
}

impl<'a> IntoFieldValue<'a, FieldValue<'a>> for f64 {
    fn into_field_value(self) -> FieldValue<'a> {
        format!("{}", self).as_bytes().to_vec().into()
    }
}

impl<'a> IntoFieldValue<'a, FieldValue<'a>> for i64 {
    fn into_field_value(self) -> FieldValue<'a> {
        format!("{}", self).as_bytes().to_vec().into()
    }
}
