use super::definitions::{
    BlockKind, CmpBlockKind, EventBlockKind, ExprBlockKind, NoopStmtBlockKind, StmtBlockKind,
    UnsupportedBlockKind,
};
use super::dt_interface::FromJsonBlock;
use super::parse_procedures::{parse_procedures_call, parse_procedures_prototype};

pub(super) fn parse_kind(
    opcode: &str,
    inputs: &serde_json::Map<String, serde_json::Value>,
    fields: &serde_json::Map<String, serde_json::Value>,
    mutation: Option<&serde_json::Map<String, serde_json::Value>>,
) -> Result<BlockKind, super::ParseKindError> {
    if let Some(unsupported) = UnsupportedBlockKind::from_json_block(opcode, inputs, fields)? {
        return Err(super::ParseKindError::OpcodeUnsupported(unsupported));
    }

    match opcode {
        "procedures_definition" => {
            return Ok(BlockKind::ProceduresDefinition {
                custom_block: super::getter!(inputs."custom_block" as blockref)?,
            });
        }
        "procedures_prototype" => return parse_procedures_prototype(opcode, inputs, mutation),
        "procedures_call" => return parse_procedures_call(opcode, inputs, mutation),

        _ => {}
    }

    Ok(
        if let Some(opt) = EventBlockKind::from_json_block(opcode, inputs, fields)? {
            opt.into()
        } else if let Some(opt) = CmpBlockKind::from_json_block(opcode, inputs, fields)? {
            opt.into()
        } else if let Some(opt) = ExprBlockKind::from_json_block(opcode, inputs, fields)? {
            opt.into()
        } else if let Some(opt) = StmtBlockKind::from_json_block(opcode, inputs, fields)? {
            opt.into()
        } else if let Some(opt) = NoopStmtBlockKind::from_json_block(opcode, inputs, fields)? {
            opt.into()
        } else {
            // typically this also means unsupported
            return Err(super::ParseKindError::OpcodeUnknown(opcode.into()));
        },
    )
}
