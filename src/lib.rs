//! A no-alloc version of [`ToString`] implemented for bool/integer/float types formatting into an [`ArrayString`].
//!
//! ## Minimum Supported Rust Version
//!
//! This is currently 1.56, and is considered a breaking update to increase.

#![no_std]
#![warn(clippy::pedantic)]

#[cfg(any(doc, test))]
extern crate alloc;

#[cfg(any(doc, test))]
use alloc::string::ToString;

use arrayvec::ArrayString;

use macros::{gen_fmt_to_buf, gen_impl};

mod erased;
mod macros;

/// A no-alloc version of [`ToString`] implemented for bool/integer/float types formatting into an [`ArrayString`].
pub trait ToArrayString: Copy {
    /// An associated type to turn [`ArrayString`]'s const generic into a type generic,
    /// working around limitations of the current type system.
    ///
    /// This is always [`ArrayString`], but in generic code this only usable as `impl Deref<Target = str>`.
    type ArrayString: erased::ArrayStringErased;

    /// Returns the value's formatted representation in an appropriately sized [`ArrayString`].
    fn to_arraystring(self) -> Self::ArrayString;
}

impl ToArrayString for bool {
    type ArrayString = ArrayString<5>;

    fn to_arraystring(self) -> Self::ArrayString {
        if self {
            ArrayString::from("true").unwrap()
        } else {
            ArrayString::from("false").unwrap()
        }
    }
}

gen_fmt_to_buf!(fmt_int_to_buf(itoa::Integer));
gen_fmt_to_buf!(fmt_float_to_buf(ryu::Float));

gen_impl!(impl_int, fmt_int_to_buf);
gen_impl!(impl_float, fmt_float_to_buf);

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
    use super::{fmt_int_to_buf, ArrayString, ToArrayString};

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
