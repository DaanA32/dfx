use crate::field_map::FieldValue;
use crate::fields::converters::TryFrom;
use crate::fields::ConversionError;

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
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
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

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}

impl<'a> TryFrom<&'a FieldValue> for f32 {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        let ref_str: &str = TryFrom::try_from(value)?;

        ref_str.parse().map_err(|_e| ConversionError::IntParseErr)
    }
}
