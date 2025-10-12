use std::collections::HashMap;

use super::definitions::BlockKind;
use crate::attr::{Expression, ProcedureArgumentDef, RefBlock};
use crate::blocks::dt_interface::ValueAttributeFromJson;
use crate::error::{BlockAttrError, BlockKindError};
use crate::{Id, StmtBlockKind, VariableValue};

macro_rules! get_procedures_json {
    ($mutation: ident, $key: literal) => {
        $mutation[$key]
            .as_str()
            .ok_or(super::ParseKindError::MissingAttr(
                $crate::error::BlockAttrError::Missing {
                    attr_name: $key.into(),
                    treated_as: "string",
                    source: "mutation",
                },
            ))
            .and_then(|text| {
                serde_json::from_str::<serde_json::Value>(text).map_err(|error| {
                    $crate::error::BlockAttrError::Invalid {
                        attr_name: $key.into(),
                        error: $crate::error::FormatError::ExpectedSerialisedJson(error.into()),
                        source: "mutation",
                        treated_as: "serialised-json",
                    }
                    .into()
                })
            })
            .and_then(|json_value| {
                json_value
                    .as_array()
                    .ok_or(
                        $crate::error::BlockKindError::Format(
                            $crate::error::FormatError::ExpectedArray.into(),
                        )
                        .into(),
                    )
                    .cloned()
            })
    };
}
pub(crate) use get_procedures_json;

pub(super) fn parse_procedures_call(
    block_kind: &str,
    inputs: &serde_json::Map<String, serde_json::Value>,

    mutation: Option<&serde_json::Map<String, serde_json::Value>>,
) -> Result<BlockKind, super::ParseKindError> {
    let mut arg_values: HashMap<Id, Expression> = HashMap::new();
    for key in inputs.keys() {
        arg_values.insert(
            key.clone().into(),
            Expression::value_from_json_outer("inputs", inputs, key.clone().into())?,
        );
    }

    let mutation = mutation.ok_or(crate::error::BlockKindError::NoMutation {
        block_kind: block_kind.to_string(),
    })?;

    let arguments = super::parse_procedures::get_procedures_json!(mutation, "argumentids")?
        .into_iter()
        .flat_map(|v| v.as_str().map(|s| s.into()))
        .map(|id: Id| (id.clone(), arg_values.get(&id).cloned()))
        .collect();

    let proccode = mutation["proccode"]
        .as_str()
        .ok_or(super::ParseKindError::MissingAttr(
            crate::error::BlockAttrError::Missing {
                attr_name: "proccode".into(),
                treated_as: "string",
                source: "mutation",
            },
        ))?
        .into();

    Ok(StmtBlockKind::ProceduresCall {
        arguments,
        // argument_values: arg_values,
        proccode,
        // argumentids,
    }
    .into())
}

pub(super) fn parse_procedures_prototype(
    block_kind: &str,
    inputs: &serde_json::Map<String, serde_json::Value>,
    mutation: Option<&serde_json::Map<String, serde_json::Value>>,
) -> Result<BlockKind, super::ParseKindError> {
    let mutation = mutation.ok_or(BlockKindError::NoMutation {
        block_kind: block_kind.to_string(),
    })?;

    let defaults = get_procedures_json!(mutation, "argumentdefaults")?;
    let ids = get_procedures_json!(mutation, "argumentids")?;
    let names = get_procedures_json!(mutation, "argumentnames")?;

    let argument_inputs: Result<HashMap<RefBlock, RefBlock>, BlockAttrError> = inputs
        .iter()
        .map(|(key, value)| {
            let value = crate::interpret_json::get_block_ref(value).map_err(|error| {
                crate::blocks::BlockAttrError::Invalid {
                    treated_as: stringify! {$elemtype},
                    attr_name: "procedures-argument".into(),
                    source: "inputs",
                    error,
                }
            })?;
            Ok((key.clone().into(), value))
        })
        .collect();
    let argument_inputs = argument_inputs?;

    let proccode = mutation["proccode"]
        .as_str()
        .ok_or(super::ParseKindError::MissingAttr(
            BlockAttrError::Missing {
                attr_name: "proccode".into(),
                treated_as: "string",
                source: "mutation",
            },
        ))?
        .into();

    if !(defaults.len() == ids.len() && ids.len() == names.len()) {
        Err(BlockKindError::Format(
            crate::error::FormatError::ArraysMustHaveSameLength.into(),
        ))?;
    }

    let arguments: Result<Vec<ProcedureArgumentDef>, crate::error::FormatError> = ids
        .into_iter()
        .zip(names.into_iter().zip(defaults))
        .enumerate()
        .map(|(index, (id, (name, default)))| {
            let id = id
                .as_str()
                .ok_or(crate::error::FormatError::MissingTextPrim(
                    index.try_into().unwrap_or(u8::MAX),
                ))?
                .into();
            let name = name
                .as_str()
                .ok_or(crate::error::FormatError::MissingTextPrim(
                    index.try_into().unwrap_or(u8::MAX),
                ))?
                .into();
            let default_value = if let Some(num) = default.as_number() {
                VariableValue::try_from(num.clone())?
            } else {
                VariableValue::Text(
                    default
                        .as_str()
                        .ok_or(crate::error::FormatError::MissingTextPrim(
                            index.try_into().unwrap_or(u8::MAX),
                        ))?
                        .into(),
                )
            };
            let def = ProcedureArgumentDef {
                reporter_id_in_this_block: argument_inputs
                    .get(&id)
                    .ok_or(crate::error::FormatError::MissingTextPrim(
                        index.try_into().unwrap_or(u8::MAX),
                    ))?
                    .clone(),
                argument_id: id,
                name,
                default_value,
            };
            Ok(def)
        })
        .collect();

    Ok(BlockKind::ProceduresPrototype {
        proccode,
        arguments: arguments.map_err(BlockKindError::from)?.into(),
    })
}
