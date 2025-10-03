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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SaturatingU16 {
    inner: u16, // Trường dữ liệu chính, lưu giá trị kiểu u16
}

// ---------- Implemented From trait for SaturatingU16 ----------
// Cho phép chuyển đổi dễ dàng từ nhiều kiểu số sang SaturatingU16

// Từ u16 -> SaturatingU16
impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 { inner: value }
    }
}

// Từ u8 -> SaturatingU16
impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        // .into() giúp tự động chuyển u8 -> u16
        SaturatingU16 { inner: value.into() }
    }
}

// Từ &u16 -> SaturatingU16
impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        // value là tham chiếu, nên cần deref (*) để lấy giá trị thực
        SaturatingU16 { inner: *value }
    }
}

// Từ &u8 -> SaturatingU16
impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 { inner: *value as u16 }
    }
}

// ---------- Implemented Add trait for SaturatingU16 ----------
// Giúp dùng toán tử + giữa nhiều kiểu khác nhau (giống operator overloading trong Java/C++)

// Cộng với giá trị u16
impl Add<u16> for SaturatingU16 {
    type Output = Self;
    
    fn add(self, rhs: u16) -> Self::Output {
        // Dùng saturating_add để tránh tràn số (overflow)
        SaturatingU16 {
            inner: self.inner.saturating_add(rhs),
        }
    }
}

// Cộng với tham chiếu &u16
impl Add<&u16> for SaturatingU16 {
    type Output = Self;
    
    fn add(self, rhs: &u16) -> Self::Output {
        SaturatingU16 {
            inner: self.inner.saturating_add(*rhs),
        }
    }
}

// Cộng với tham chiếu &SaturatingU16
impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        SaturatingU16 {
            inner: self.inner.saturating_add(rhs.inner),
        }
    }
}

// Cộng với chính kiểu SaturatingU16
impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: SaturatingU16) -> Self::Output {
        SaturatingU16 {
            inner: self.inner.saturating_add(rhs.inner),
        }
    }
}

// ---------- Implemented PartialEq trait for SaturatingU16 ----------
// Cho phép so sánh trực tiếp SaturatingU16 với u16 hoặc ngược lại (== hoạt động cả hai chiều)

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.inner == *other
    }
}

impl PartialEq<SaturatingU16> for u16 {
    fn eq(&self, other: &SaturatingU16) -> bool {
        *self == other.inner
    }
}
