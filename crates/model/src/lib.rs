/// copied from [<https://github.com/scratchfoundation/scratch-vm/blob/develop/src/serialization/sb3.js>]
pub mod constants;

mod blocks;
mod error;
mod ext;
mod interpret_json;
mod reader;
mod scopes;
mod scratch_expr;
mod types;

pub use blocks::{
    BlockAttrError, BlockKind, BlockKindError, CmpBlockKind, CmpBlockKindUnit, EventBlockKind,
    ExprBlockKind, StmtBlockKind, UnsupportedBlockKind,
};

use interpret_json::FormatError;

pub use error::{DocError, Error};
pub use interpret_json::{Expression, List, Variable};
pub use reader::json_from_sb3_stream;
pub use scopes::*;
pub use scratch_expr::{IntegerOutOfBounds, SValue, SValue as VariableValue, ScratchExpr};
pub use types::{ArgumentReporterName, DropdownSelection, Id, OpcodeNum, RefBlock};
