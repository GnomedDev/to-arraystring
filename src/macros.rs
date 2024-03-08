macro_rules! gen_fmt_to_buf {
    ($name:ident($lib:ident::$type:ident)) => {
        fn $name<const CAP: usize>(val: impl $lib::$type) -> ArrayString<CAP> {
            let mut buffer = $lib::Buffer::new();
            let formatted_ref = buffer.format(val);

            ArrayString::from(formatted_ref).unwrap()
        }
    };
}

macro_rules! gen_impl {
    ($name:ident, $body:path) => {
        macro_rules! $name {
            (ToArrayString<$len:literal> for $type:ty) => {
                impl ToArrayString for $type {
                    type ArrayString = ArrayString<$len>;
                    fn to_arraystring(self) -> Self::ArrayString {
                        $body(self)
                    }
                }
            };
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
            }
        )*);
    };
}

pub(crate) use {gen_fmt_to_buf, gen_impl};

#[cfg(test)]
pub(crate) use generate_test;
