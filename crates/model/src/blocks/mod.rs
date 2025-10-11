mod _macros;
mod datatypes;
mod definitions;
mod dt_interface;
mod error;
mod implementations;
mod parsing;

use _macros::define_blocks;
use error::ParseKindError;
use parsing::parse_kind;

pub use definitions::{
    BlockKind, CmpBlockKind, EventBlockKind, ExprBlockKind, StmtBlockKind, UnsupportedBlockKind,
};

#[allow(unused)]
pub use definitions::{
    BlockKindUnit, CmpBlockKindUnit, EventBlockKindUnit, ExprBlockKindUnit, StmtBlockKindUnit,
    UnsupportedBlockKindUnit,
};

pub use error::{BlockAttrError, BlockKindError};
