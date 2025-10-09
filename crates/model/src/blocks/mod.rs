mod _macros;
mod datatypes;
mod definitions;
mod dt_interface;
mod error;
mod implementations;

use _macros::define_blocks;
use definitions::parse_kind;
use error::ParseKindError;

pub use definitions::{
    BlockKind, CmpBlockKind, CmpBlockKindUnit, EventBlockKind, ExprBlockKind, StmtBlockKind,
};
pub use error::{BlockAttrError, BlockKindError};
