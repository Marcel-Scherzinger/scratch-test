mod def_cmps;
mod def_events;
mod def_exprs;
mod def_stmts;
mod no_op;
mod unsupported;
pub use def_cmps::{CmpBlockKind, CmpBlockKindUnit};
pub use def_events::{EventBlockKind, EventBlockKindUnit};
pub use def_exprs::{ExprBlockKind, ExprBlockKindUnit};
pub use def_stmts::{StmtBlockKind, StmtBlockKindUnit};
pub use no_op::{NoopStmtBlockKind, NoopStmtBlockKindUnit};
pub use unsupported::UnsupportedBlockKind;

use super::define_blocks;
use super::dt_interface::FromJsonBlock;
use crate::blocks::dt_interface::GetOpcodeUnit;
#[allow(unused)]
use crate::{
    ArgumentReporterName, DropdownSelection, Expression, RefBlock, Variable,
    interpret_json::{FormatError, List},
};

#[derive(Debug, derive_more::From, PartialEq)]
pub enum BlockKind {
    Event(EventBlockKind),
    Cmp(CmpBlockKind),
    Expr(ExprBlockKind),
    Stmt(StmtBlockKind),
    Noop(NoopStmtBlockKind),
}

#[derive(Debug, derive_more::From, PartialEq, Clone, Copy, derive_more::Display)]
pub enum BlockKindUnit {
    #[display("{_0}")]
    Event(EventBlockKindUnit),
    #[display("{_0}")]
    Cmp(CmpBlockKindUnit),
    #[display("{_0}")]
    Expr(ExprBlockKindUnit),
    #[display("{_0}")]
    Stmt(StmtBlockKindUnit),
    #[display("{_0}")]
    Noop(NoopStmtBlockKindUnit),
}

impl GetOpcodeUnit for BlockKind {
    type Opcode = BlockKindUnit;

    fn get_opcode(&self) -> Self::Opcode {
        match self {
            Self::Expr(u) => u.get_opcode().into(),
            Self::Event(u) => u.get_opcode().into(),
            Self::Cmp(u) => u.get_opcode().into(),
            Self::Stmt(u) => u.get_opcode().into(),
            Self::Noop(u) => u.get_opcode().into(),
        }
    }
}

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
