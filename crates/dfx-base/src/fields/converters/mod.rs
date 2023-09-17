pub mod datetime;
pub use datetime::{DateTime, Time, Date};
mod string;
pub use string::*;

mod int;
pub mod r#bool;

mod decimal;
pub use decimal::*;

use crate::field_map::FieldValue;

pub trait TryFrom<T>
where
    Self: Sized,
{
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait IntoFieldValue<'a, T>
where
    Self: Sized,
{
    fn into_field_value(&'a self) -> T;
}

impl<'a> IntoFieldValue<'a, FieldValue<'a>> for &std::sync::Arc<[u8]> {
    fn into_field_value(&self) -> FieldValue<'a> {
        self.to_vec().into()
    }
}

impl<'a> IntoFieldValue<'a, FieldValue<'a>> for &std::borrow::Cow<'_, [u8]> {
    fn into_field_value(&self) -> FieldValue<'a> {
        self.to_vec().into()
    }
}
