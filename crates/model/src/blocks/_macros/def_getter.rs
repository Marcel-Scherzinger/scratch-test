/// mostly replaced by block definition macro,
/// now it's only used by own blocks
macro_rules! getter {
    ($map: ident . $key: literal as $elemtype: ident) => {
        $crate::blocks::getter!($map: $map . $key as $elemtype, )
    };
    ($map: ident . $key: literal as optional $elemtype: ident) => {
        $crate::blocks::getter!($map: $map . $key as $elemtype, optional)
    };
    (inputs: $map: ident . $key: literal as $elemtype: ident, $($modifier: ident)?) => {{
        $crate::blocks::getter!(/"inputs": $map.$key as $elemtype, $($modifier)?)
    }};
    (fields: $map: ident . $key: literal as $elemtype: ident, $($modifier: ident)?) => {{
        $crate::blocks::getter!(/"fields": $map.$key as $elemtype, $($modifier)?)
    }};
    (/$source: literal: $map: ident . $key: literal as $elemtype: ident, ) => {{
        if let Some(entry) = &$map.get($key) {
            $crate::blocks::getter!(;; $elemtype, &entry).map_err(|error| {
                $crate::blocks::BlockAttrError::Invalid {
                    treated_as: stringify!{$elemtype}, attr_name: $key.into(), source: $source, error
                }
            })
        } else {
            Err($crate::blocks::BlockAttrError::Missing{
                treated_as: stringify!{$elemtype}, attr_name: $key.into(), source: $source,
            })
        }
    }};
    (/$source: literal: $map: ident . $key: literal as $elemtype: ident, optional) => {{
        if let Some(entry) = &$map.get($key) {
            match $crate::blocks::getter!(;; $elemtype, &entry) {
                Ok(o) => Ok(Some(o)),
                Err($crate::interpret_json::FormatError::OpcodeNull) => Ok(None),
                Err(error) => {
                    Err($crate::blocks::BlockAttrError::Invalid {
                        treated_as: stringify!{$elemtype}, attr_name: $key.into(), source: $source, error
                    })
                }
            }
        } else {
            Ok(None)
        }
    }};
    (;; blockref, $arg: expr) => { crate::interpret_json::get_block_ref($arg) };
    (;; listref, $arg: expr) => { crate::interpret_json::get_list_ref($arg) };
    (;; variableref, $arg: expr) => { crate::interpret_json::get_variable_ref($arg) };
    (;; expression, $arg: expr) => { crate::interpret_json::get_expression($arg) };
    (;; dropdown, $arg: expr) => { crate::interpret_json::get_dropdown_selection($arg) };
    (;; argumentreporter, $arg: expr) => { crate::interpret_json::get_argument_reporter_name($arg) };
}
pub(crate) use getter;
