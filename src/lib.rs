//! A no-alloc version of [`ToString`] implemented for bool/integer/float types formatting into an [`ArrayString`].
//!
//! ## Minimum Supported Rust Version
//!
//! This is currently 1.56, and is considered a breaking update to increase.
//!
//! - Using the `nonzero_impls` feature, this increases to 1.79.

#![no_std]
#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports)]

#[cfg(any(doc, test))]
extern crate alloc;

#[cfg(any(doc, test))]
use alloc::string::ToString;

pub use arrayvec::ArrayString;

use macros::{fmt_float_to_buf, fmt_int_to_buf, impl_float, impl_int};

mod erased;
mod macros;

/// A no-alloc version of [`ToString`] implemented for bool/integer/float types formatting into an [`ArrayString`].
pub trait ToArrayString: Copy {
    /// The maximum length that Self can be formatted into. This is used for the capacity generic of [`ArrayString`].
    ///
    /// # Note for implementors
    /// This must match the capacity generic used in [`ArrayString`], otherwise logic bugs and panics may occur.
    const MAX_LENGTH: usize;

    /// An associated type to turn [`ArrayString`]'s const generic into a type generic,
    /// working around limitations of the current type system.
    ///
    /// This is always [`ArrayString`], but in generic code this only usable as `impl Deref<Target = str>`.
    type ArrayString: erased::ArrayStringErased;

    /// Returns the value's formatted representation in an appropriately sized [`ArrayString`].
    fn to_arraystring(self) -> Self::ArrayString;
}

impl<const MAX_LENGTH: usize> ToArrayString for ArrayString<MAX_LENGTH> {
    const MAX_LENGTH: usize = MAX_LENGTH;
    type ArrayString = ArrayString<MAX_LENGTH>;

    #[inline]
    fn to_arraystring(self) -> Self::ArrayString {
        self
    }
}

impl ToArrayString for char {
    const MAX_LENGTH: usize = 4;
    type ArrayString = ArrayString<4>;

    #[inline]
    fn to_arraystring(self) -> Self::ArrayString {
        let mut buffer = [0; 4];
        let char_str = self.encode_utf8(&mut buffer);

        ArrayString::from(char_str).unwrap()
    }
}

impl ToArrayString for bool {
    const MAX_LENGTH: usize = 5;
    type ArrayString = ArrayString<5>;

    #[inline]
    fn to_arraystring(self) -> Self::ArrayString {
        if self {
            ArrayString::from("true").unwrap()
        } else {
            ArrayString::from("false").unwrap()
        }
    }
}

impl_float!(ToArrayString<16> for f32);
impl_float!(ToArrayString<24> for f64);

impl_int!(ToArrayString<3> for u8);
impl_int!(ToArrayString<4> for i8);
impl_int!(ToArrayString<5> for u16);
impl_int!(ToArrayString<6> for i16);
impl_int!(ToArrayString<10> for u32);
impl_int!(ToArrayString<11> for i32);
impl_int!(ToArrayString<20> for u64);
impl_int!(ToArrayString<21> for i64);
impl_int!(ToArrayString<39> for u128);
impl_int!(ToArrayString<40> for i128);

#[cfg(target_pointer_width = "16")]
mod usize_impls {
    use super::*;

    impl_int!(ToArrayString<5> for usize);
    impl_int!(ToArrayString<6> for isize);
}

#[cfg(target_pointer_width = "32")]
mod usize_impls {
    use super::*;

    impl_int!(ToArrayString<10> for usize);
    impl_int!(ToArrayString<11> for isize);
}

#[cfg(target_pointer_width = "64")]
mod usize_impls {
    use super::*;

    impl_int!(ToArrayString<20> for usize);
    impl_int!(ToArrayString<21> for isize);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_impl<T: ToArrayString + ToString>(min: T, max: T) {
        assert_eq!(&*min.to_arraystring(), min.to_string());
        assert_eq!(&*max.to_arraystring(), max.to_string());
    }

    crate::macros::generate_test!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
}
