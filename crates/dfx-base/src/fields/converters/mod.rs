pub mod datetime;
pub use datetime::{Date, DateTime, Time};
mod string;

pub mod r#bool;
mod int;

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

pub trait IntoBytes<T>
where
    Self: Sized,
{
    fn as_bytes(&self) -> T;
}

impl IntoBytes<FieldValue> for &std::sync::Arc<[u8]> {
    fn as_bytes(&self) -> FieldValue {
        self.to_vec().into()
    }
}
