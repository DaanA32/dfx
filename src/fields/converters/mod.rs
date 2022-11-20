pub mod datetime;
mod string;
pub use string::*;
mod int;

pub trait TryFrom<T>
where
    Self: Sized,
{
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}
