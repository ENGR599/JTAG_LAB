use std::ops::Mul;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bits<T>(pub T);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bytes<T>(pub T);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Words32<T>(pub T);

impl<T> From<Bytes<T>> for Bits<T>
where
    T: Mul<usize, Output = T>,
{
    fn from(value: Bytes<T>) -> Self {
        Self(value.0 * 8)
    }
}

impl<T> From<Words32<T>> for Bytes<T>
where
    T: Mul<usize, Output = T>,
{
    fn from(value: Words32<T>) -> Self {
        Self(value.0 * 4)
    }
}
