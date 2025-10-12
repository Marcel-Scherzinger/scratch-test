macro_rules! impl_string_from {
    ($type: ty, $inter: ty) => {
        impl<'a> From<&'a str> for $type {
            fn from(val: &'a str) -> Self {
                let r: $inter = val.into();
                r.into()
            }
        }
        impl<'a> From<String> for $type {
            fn from(val: String) -> Self {
                let r: $inter = val.into();
                r.into()
            }
        }
    };
}
pub(crate) use impl_string_from;
