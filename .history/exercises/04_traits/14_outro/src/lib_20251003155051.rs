// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct SaturatingU16 {
    inner: u16,
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 { inner: value }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;
    
    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16 {
            inner: self.inner.saturating_add(rhs),
        }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;
    
    fn add(self, rhs: &u16) -> Self::Output {
        SaturatingU16 {
            inner: self.inner.saturating_add(*rhs),
        }
    }
}

impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        SaturatingU16 {
            inner: self.inner.saturating_add(rhs.inner),
        }
    }
}