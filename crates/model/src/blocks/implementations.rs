use crate::{
    BlockKind,
    blocks::{BlockKindError, ExprBlockKind, ParseKindError},
    constants,
    interpret_json::{List, Variable},
};

impl crate::ext::FromJsonExt<Self, BlockKindError> for BlockKind {
    fn from_json_without_ctx(obj: &serde_json::Value) -> Result<Self, BlockKindError> {
        if obj.is_object() {
            let opcode = obj["opcode"].as_str().ok_or(BlockKindError::NoOpcode)?;
            let inputs = obj["inputs"]
                .as_object()
                .ok_or_else(|| BlockKindError::NoInputs {
                    block_kind: opcode.to_string(),
                })?;
            let fields = obj["fields"]
                .as_object()
                .ok_or_else(|| BlockKindError::NoFields {
                    block_kind: opcode.to_string(),
                })?;
            let mutation = obj["mutation"].as_object();

            match super::parse_kind(opcode, inputs, fields, mutation) {
                Err(ParseKindError::OpcodeUnknown(opcode)) => {
                    Err(BlockKindError::UnknownBlock(opcode))
                }
                Err(ParseKindError::BlockKind(err)) => Err(err),
                Err(ParseKindError::OpcodeUnsupported(opcode)) => {
                    Err(BlockKindError::UnsupportedBlock(opcode))
                }
                Err(ParseKindError::MissingAttr(error)) => Err(BlockKindError::Missing {
                    block_kind: opcode.to_string(),
                    error: error.into(),
                }),
                Ok(b) => Ok(b),
            }
        } else if obj.is_array() {
            // variable or list reading block
            let opcode_num = crate::interpret_json::get_opcode(obj)
                .map_err(|_| BlockKindError::NoOpcodeInArr)?;
            use constants::*;
            match opcode_num {
                VAR_PRIMITIVE => {
                    let variable: Variable = crate::interpret_json::get_variable_ref(obj)?;
                    Ok(ExprBlockKind::RDataVar { variable }.into())
                }
                LIST_PRIMITIVE => {
                    let list: List = crate::interpret_json::get_list_ref(obj)?;
                    Ok(ExprBlockKind::RDataList { list }.into())
                }
                _ => Err(BlockKindError::InvalidBlockType),
            }
        } else {
            Err(BlockKindError::InvalidBlockType)
        }
    }
}
