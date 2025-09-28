mod _macros;
mod definitions;
mod error;
mod implementations;

use _macros::getter;
use definitions::parse_kind;
use error::ParseKindError;

pub use definitions::{BlockKind, CmpBlockKind, ExprBlockKind, StmtBlockKind};
pub use error::{BlockAttrError, BlockKindError};
