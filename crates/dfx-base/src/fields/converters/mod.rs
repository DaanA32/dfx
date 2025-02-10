pub mod datetime;
pub use datetime::{Date, DateTime, Time};
mod string;

pub mod r#bool;
mod int;

mod decimal;
pub use decimal::*;

use crate::field_map::FieldValue;

pub trait TryFromFieldValue<T>
where
    Self: Sized,
{
    type Error;
    fn try_from_field_value(value: T) -> Result<Self, Self::Error>;
}

pub trait IntoFieldValue<T>
where
    Self: Sized,
{
    fn into_field_value(&self) -> T;
}

impl IntoFieldValue<FieldValue> for &std::sync::Arc<[u8]> {
    fn into_field_value(&self) -> FieldValue {
        self.to_vec().into()
    }
}
