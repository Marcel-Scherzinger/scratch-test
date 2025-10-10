macro_rules! def_dt_impl {
    ($($type: ty : $name: literal $( { $($token: tt)+ } )?),* $(,)?) => {
        $(
            impl crate::blocks::dt_interface::ValueAttrJsonElemtype for $type {
                const ELEMTYPE: &'static str = $name;
            }
            $(
                impl crate::blocks::dt_interface::ValueAttributeFromJson for $type {
                    fn value_from_json(value: &serde_json::Value) -> Result<Self, FormatError>
                    where
                        Self: Sized,
                    {
                        crate::blocks::_macros::def_dt_impl!(;value; $($token)*)
                    }
                }
            )?
        )*
    };
    (;$input: ident;|$value: ident| $block: block) => {
        (|$value| $block)($input)
    };

    (;$input: ident;|$value: ident| $block: expr) => {
        (|$value| $block)($input)
    };

    (;$input: ident;$func: path) => {
        $func($input)
    };

}
pub(crate) use def_dt_impl;
