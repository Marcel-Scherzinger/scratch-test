use super::definitions::{
    BlockKind, CmpBlockKind, EventBlockKind, ExprBlockKind, NoopStmtBlockKind, StmtBlockKind,
    UnsupportedBlockKind,
};
use super::dt_interface::FromJsonBlock;

pub(super) fn parse_kind(
    opcode: &str,
    inputs: &serde_json::Map<String, serde_json::Value>,
    fields: &serde_json::Map<String, serde_json::Value>,
) -> Result<BlockKind, super::ParseKindError> {
    if let Some(unsupported) = UnsupportedBlockKind::from_json_block(opcode, inputs, fields)? {
        return Err(super::ParseKindError::OpcodeUnsupported(unsupported));
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
