use super::{CmpBlockKind, EventBlockKind, ExprBlockKind, NoopStmtBlockKind, StmtBlockKind};
use super::{
    CmpBlockKindUnit, EventBlockKindUnit, ExprBlockKindUnit, NoopStmtBlockKindUnit,
    StmtBlockKindUnit, UnsupportedBlockKind, UnsupportedBlockKindUnit,
};
use crate::blocks::dt_interface::GetOpcodeUnit;

/// opcode block type is [`BlockKindUnit`]
#[derive(Debug, derive_more::From, PartialEq)]
pub enum BlockKind {
    Event(EventBlockKind),
    Cmp(CmpBlockKind),
    Expr(ExprBlockKind),
    Stmt(StmtBlockKind),
    Noop(NoopStmtBlockKind),
    Unsup(UnsupportedBlockKind),
}

/// main block type is [`BlockKind`]
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
    #[display("{_0}")]
    Unsup(UnsupportedBlockKindUnit),
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
            Self::Unsup(u) => u.get_opcode().into(),
        }
    }
}
