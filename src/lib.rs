//! A no-alloc version of [`alloc::string::ToString`] implemented for integer types formatting into an [`ArrayString`].
#![no_std]

use arrayvec::ArrayString;

mod erased;

/// A no-alloc version of [`alloc::string::ToString`] implemented for integer types formatting into an [`ArrayString`].
pub trait ToArrayString: Copy {
    type ArrayString: erased::ArrayStringErased;
    fn to_arraystring(self) -> Self::ArrayString;
}

fn fmt_to_buf<const CAP: usize>(val: impl itoa::Integer) -> ArrayString<CAP> {
    let mut buffer = itoa::Buffer::new();
    let formatted_ref = buffer.format(val);

    ArrayString::from(formatted_ref).unwrap()
}

macro_rules! impl_arraystr {
    ($(impl ToArrayString<$len:literal> for $type:ty;)*) => {
        $(
            impl ToArrayString for $type {
                type ArrayString = ArrayString<$len>;
                fn to_arraystring(self) -> Self::ArrayString {
                    fmt_to_buf(self)
                }
            }
        )*
    };
}

impl_arraystr!(
    impl ToArrayString<3> for u8;
    impl ToArrayString<4> for i8;
    impl ToArrayString<5> for u16;
    impl ToArrayString<6> for i16;
    impl ToArrayString<10> for u32;
    impl ToArrayString<11> for i32;
    impl ToArrayString<20> for u64;
    impl ToArrayString<21> for i64;
    impl ToArrayString<39> for u128;
    impl ToArrayString<40> for i128;
);

#[cfg(target_pointer_width = "16")]
impl_arraystr!(
    impl ToArrayString<5> for usize;
    impl ToArrayString<6> for isize;
);

#[cfg(target_pointer_width = "32")]
impl_arraystr!(
    impl ToArrayString<10> for usize;
    impl ToArrayString<11> for isize;
);

#[cfg(target_pointer_width = "64")]
impl_arraystr!(
    impl ToArrayString<20> for usize;
    impl ToArrayString<21> for isize;
);

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::string::ToString;

    use crate::ToArrayString;

    fn test_impl<T: ToArrayString + ToString>(min: T, max: T) {
        assert_eq!(&*min.to_arraystring(), min.to_string());
        assert_eq!(&*max.to_arraystring(), max.to_string());
    }

    #[test]
    fn check_u8() {
        test_impl(u8::MIN, u8::MAX);
        test_impl(i8::MIN, i8::MAX);
    }

    #[test]
    fn check_16() {
        test_impl(u16::MIN, u16::MAX);
        test_impl(i16::MIN, i16::MAX);
    }

    #[test]
    fn check_32() {
        test_impl(u32::MIN, u32::MAX);
        test_impl(i32::MIN, i32::MAX);
    }

    #[test]
    fn check_64() {
        test_impl(u64::MIN, u64::MAX);
        test_impl(i64::MIN, i64::MAX);
    }

    #[test]
    fn check_size() {
        test_impl(usize::MIN, usize::MAX);
        test_impl(isize::MIN, isize::MAX);
    }
}
