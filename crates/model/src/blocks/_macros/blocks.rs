macro_rules! define_blocks {
    (
        $(#[$tmeta: meta])*
        $tvis: vis enum $tname: ident  $(($unit: ident))?:

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
                    ($skipopcode: literal) $skipvar: ident $({
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
        $(#[doc = concat!("\nopcode block type is [`", stringify!{$unit}, "`]")])?
        $tvis enum $tname {
            $(
                $(#[$vmeta])*
                #[doc = concat!{"\n≡ ", $opcode}]
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

        crate::blocks::define_blocks!(
            ;unit;$tname;$tvis ($($unit)?);
            $(
                $opcode => $var $(
                    { $($fname),* }
                )?,
            )*
            $(
                $(
                    $skipopcode => $skipvar $(
                        { $($skipfname),* }
                    )?,
                )*
            )?
        );

        impl crate::blocks::dt_interface::FromJsonBlock for $tname {
            #[allow(unused)]
            fn from_json_block(
                opcode: &str,
                inputs: &serde_json::Map<String, serde_json::Value>,
                fields: &serde_json::Map<String, serde_json::Value>,
            ) -> Result<Option<Self>, crate::blocks::ParseKindError> {
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
                            })?,
                        )*
                        _ => { return Ok(None); }
                    }
                ))
            }
        }
    };
    (;unit;$name: ident;$tvis: vis ($unit: ident);
        $($opcode: literal => $var: ident $({ $($args: ident),* })?),* $(,)?
    ) => {
        #[doc = concat!("\nmain block type is [`", stringify!{$name}, "`]")]
        #[allow(unused)]
        #[derive(Debug, PartialEq, Clone, Copy, Hash)]
        $tvis enum $unit {
            $(
                #[doc = concat!{"\n≡ ", $opcode}]
                $var
            ),*
        }
        impl crate::blocks::dt_interface::GetOpcodeUnit for $name {
            type Opcode = $unit;

            fn get_opcode(&self) -> $unit {
                #[allow(unused)]
                match self {
                    $(
                        Self::$var $({ $($args),* })? => $unit::$var,
                    )*
                }
            }
        }

        impl std::fmt::Display for $unit {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$var => f.write_str($opcode),
                    )*
                }
            }
        }

    };
    (;unit;$name: ident;$tvis: vis ();
        $($opcode: literal => $var: ident $({ $($args : ident),* })?),* $(,)?
    ) => {};
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
