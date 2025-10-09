macro_rules! define_blocks {
    (
        $(#[$tmeta: meta])*
        $tvis: vis enum $tname: ident :

        $(
            $(#[$vmeta: meta])*
            $opcode: literal => $var: ident $({
                $(
                    $(#[$fmeta: meta])*
                    $(($fsrc: ident))?  $fname : ident $($fkey: literal)? : $ftype: ty
                ),*
                $(,)?
            })?
        ),*

        $(
        ,
            skip => {
                $(
                    $(#[$skipvmeta: meta])*
                    $skipvar: ident $({
                        $(
                            $(#[$skipfmeta: meta])*
                            $skipfname : ident : $skipftype: ty
                        ),*
                        $(,)?
                    })?
                ),*
                $(,)?
            }
        )?

        $(,)?
    ) => {
        $(#[$tmeta])*
        $tvis enum $tname {
            $(
                $(#[$vmeta: meta])*
                $var $({
                    $(
                        $(#[$fmeta])*
                        $fname: $ftype
                    ),*
                })?
            ),*
            $(,
                $(
                    $(#[$skipvmeta])*
                    $skipvar $( {
                        $(
                            $(#[$skipfmeta])*
                            $skipfname: $skipftype
                        ),*
                    } )?
                ),*
            )?

        }

        impl crate::blocks::dt_interface::FromJsonBlock for $tname {
            #[allow(unused)]
            fn from_json_block(
                opcode: &str,
                inputs: &serde_json::Map<String, serde_json::Value>,
                fields: &serde_json::Map<String, serde_json::Value>,
            ) -> Result<Option<crate::BlockKind>, crate::blocks::ParseKindError> {
                use crate::blocks::dt_interface::ValueAttributeFromJson;
                Ok(Some(
                    match opcode {
                        $(
                            $opcode => Self::$var $({
                                $(
                                    $fname: <$ftype as ValueAttributeFromJson>::value_from_json_outer(
                                        crate::blocks::define_blocks!(;get_obj_str;$($fsrc)?; inputs, fields),
                                        crate::blocks:: define_blocks!(;get_obj;$($fsrc)?; inputs, fields),
                                        crate::blocks:: define_blocks!(;get_key;; $($fkey)?, $fname)
                                    )?,
                                )*
                            })?.into(),
                        )*
                        _ => { return Ok(None); }
                    }
                ))
            }
        }
    };
    (;get_obj;inputs; $inputs: ident, $fields: ident) => { $inputs };
    (;get_obj;fields; $inputs: ident, $fields: ident) => { $fields };
    (;get_obj;input; $inputs: ident, $fields: ident) => { $inputs };
    (;get_obj;field; $inputs: ident, $fields: ident) => { $fields };
    (;get_obj;; $inputs: ident, $fields: ident) => { $inputs };

    (;get_obj_str;inputs; $inputs: ident, $fields: ident) => { "inputs" };
    (;get_obj_str;fields; $inputs: ident, $fields: ident) => { "fields" };
    (;get_obj_str;input; $inputs: ident, $fields: ident) => { "inputs" };
    (;get_obj_str;field; $inputs: ident, $fields: ident) => { "fields" };
    (;get_obj_str;; $inputs: ident, $fields: ident) => { "inputs" };

    (;get_key;; $fkey: literal, $fname: ident) => { $fkey };
    (;get_key;; , $fname: ident) => { {
        const {
            const_str::convert_ascii_case!{upper, stringify! { $fname } }
        }
    } };
}
pub(crate) use define_blocks;
