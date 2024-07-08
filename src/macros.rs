macro_rules! gen_fmt_to_buf {
    ($name:ident($lib:ident::$type:ident)) => {
        pub(crate) fn $name<const CAP: usize>(val: impl $lib::$type) -> arrayvec::ArrayString<CAP> {
            let mut buffer = $lib::Buffer::new();
            let formatted_ref = buffer.format(val);

            arrayvec::ArrayString::from(formatted_ref).unwrap()
        }
    };
}

gen_fmt_to_buf!(fmt_int_to_buf(itoa::Integer));
gen_fmt_to_buf!(fmt_float_to_buf(ryu::Float));

macro_rules! impl_int {
    (ToArrayString<$len:literal> for $type:ty) => {
        impl ToArrayString for $type {
            const MAX_LENGTH: usize = $len;
            type ArrayString = ArrayString<$len>;

            #[inline]
            fn to_arraystring(self) -> Self::ArrayString {
                fmt_int_to_buf(self)
            }
        }

        impl ToArrayString for core::num::NonZero<$type> {
            const MAX_LENGTH: usize = $len;
            type ArrayString = ArrayString<$len>;

            #[inline]
            fn to_arraystring(self) -> Self::ArrayString {
                fmt_int_to_buf(self.get())
            }
        }
    };
}

macro_rules! impl_float {
    (ToArrayString<$len:literal> for $type:ty) => {
        impl ToArrayString for $type {
            const MAX_LENGTH: usize = $len;
            type ArrayString = ArrayString<$len>;

            #[inline]
            fn to_arraystring(self) -> Self::ArrayString {
                fmt_float_to_buf(self)
            }
        }
    };
}
#[cfg(test)]
macro_rules! generate_test {
    ($($type:ident),*) => {
        paste::paste!($(
            #[test]
            fn [<check_ $type>]() {
                test_impl($type::MIN, $type::MAX);
                test_impl(core::num::NonZero::<$type>::MIN, core::num::NonZero::<$type>::MAX);
            }
        )*);
    };
}

pub(crate) use {impl_float, impl_int};

#[cfg(test)]
pub(crate) use generate_test;
