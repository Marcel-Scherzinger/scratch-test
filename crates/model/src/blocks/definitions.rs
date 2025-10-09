mod def_cmps;
mod def_events;
mod def_exprs;
mod def_stmts;
pub use def_cmps::{CmpBlockKind, CmpBlockKindUnit};
pub use def_events::{EventBlockKind, EventBlockKindUnit};
pub use def_exprs::{ExprBlockKind, ExprBlockKindUnit};
pub use def_stmts::{StmtBlockKind, StmtBlockKindUnit};

use super::define_blocks;
use super::dt_interface::FromJsonBlock;
use crate::blocks::dt_interface::GetOpcodeUnit;
#[allow(unused)]
use crate::{
    ArgumentReporterName, DropdownSelection, Expression, RefBlock, Variable,
    interpret_json::{FormatError, List},
};

define_blocks! {
    #[derive(Debug, PartialEq)]
    pub enum NoopStmtBlockKind (NoopStmtBlockKindUnit):

    "data_showvariable" => DataShowvariable,
    "data_showlist" => DataShowlist,
    "data_hidevariable" => DataHidevariable,
    "data_hidelist" => DataHidelist,
    "looks_show" => LooksShow,
}

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
    match opcode {
        "operator_random"
        | "seinsing_touchingcolor"
        | "sensing_coloristouchingcolor"
        | "control_delete_this_clone"
        | "control_create_clone_of"
        | "control_start_as_clone"
        | "sound_sounds_menu"
        | "sensing_dayssince2000"
        | "sensing_current"
        | "sensing_timer" => {
            return Err(super::ParseKindError::OpcodeUnsupported(opcode.into()));
        }
        _ => {}
    }

    Ok(
        if let Some(opt) = EventBlockKind::from_json_block(opcode, inputs, fields)? {
            opt
        } else if let Some(opt) = CmpBlockKind::from_json_block(opcode, inputs, fields)? {
            opt
        } else if let Some(opt) = ExprBlockKind::from_json_block(opcode, inputs, fields)? {
            opt
        } else if let Some(opt) = StmtBlockKind::from_json_block(opcode, inputs, fields)? {
            opt
        } else if let Some(opt) = NoopStmtBlockKind::from_json_block(opcode, inputs, fields)? {
            opt
        } else {
            // typically this also means unsupported
            return Err(super::ParseKindError::OpcodeUnknown(opcode.to_string()));
        },
    )
}
