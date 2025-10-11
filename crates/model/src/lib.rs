/// copied from [<https://github.com/scratchfoundation/scratch-vm/blob/develop/src/serialization/sb3.js>]
pub mod constants;

mod blocks;
pub mod error;
mod ext;
mod interpret_json;
mod reader;
mod scopes;
mod scratch_expr;

pub use blocks::{
    BlockKind, CmpBlockKind, CmpBlockKindUnit, EventBlockKind, ExprBlockKind, StmtBlockKind,
    UnsupportedBlockKind,
};

pub type Id = std::rc::Rc<str>;
pub use interpret_json::OpcodeNum;

pub use error::{DocError, Error};
pub use reader::json_from_sb3_stream;
pub use scopes::*;
pub use scratch_expr::{SValue, SValue as VariableValue, ScratchExpr};

pub mod attr {
    pub use crate::interpret_json::{
        ArgumentReporterName, DropdownSelection, Expression, List, RefBlock, Variable,
    };
}
