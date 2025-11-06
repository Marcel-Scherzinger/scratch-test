//! Model to parse the [file format](https://en.scratch-wiki.info/wiki/Scratch_File_Format)
//! of the [Scratch](https://scratch.mit.edu/) block-oriented programming language
//!
//! (I want to note that I was unable to find the above link to the format specification
//! when I was developing this so I reverse-engineered the format from example files.
//! Luckily, it looks like I correctly understood the meaning of the components.)
//!
//!

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
    BlockKind, CmpBlockKind, EventBlockKind, ExprBlockKind, NoopStmtBlockKind, StmtBlockKind,
    UnsupportedBlockKind,
};

pub mod block_opcodes {
    pub use crate::blocks::{
        BlockKindUnit, CmpBlockKindUnit, EventBlockKindUnit, ExprBlockKindUnit,
        NoopStmtBlockKindUnit, StmtBlockKindUnit, UnsupportedBlockKindUnit,
    };
}

pub type Id = std::rc::Rc<str>;
pub use interpret_json::OpcodeNum;

pub use error::{DocError, Error};
pub use reader::json_from_sb3_stream;
pub use scopes::*;
pub use scratch_expr::{SValue, SValue as VariableValue, ScratchExpr};

pub mod attr {
    pub use crate::interpret_json::{
        ArgumentReporterName, DropdownSelection, Expression, List, ProcedureArgumentDef, RefBlock,
        Variable,
    };
}
