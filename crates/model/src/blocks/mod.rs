mod _macros;
mod datatypes;
mod definitions;
mod dt_interface;
mod error;
mod implementations;
mod parse_procedures;
mod parsing;

use _macros::{define_blocks, getter};
use error::ParseKindError;
use parsing::parse_kind;

#[allow(unused)]
pub use definitions::{
    BlockKind, CmpBlockKind, EventBlockKind, ExprBlockKind, NoopStmtBlockKind, StmtBlockKind,
    UnsupportedBlockKind,
};

#[allow(unused)]
pub use definitions::{
    BlockKindUnit, CmpBlockKindUnit, EventBlockKindUnit, ExprBlockKindUnit, NoopStmtBlockKindUnit,
    StmtBlockKindUnit, UnsupportedBlockKindUnit,
};

pub use error::{BlockAttrError, BlockKindError};

pub use dt_interface::GetOpcodeUnit;
