pub mod datetime;
mod string;
pub use string::*;

use crate::field_map::FieldValue;

pub trait TryFrom<T> where Self: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}
