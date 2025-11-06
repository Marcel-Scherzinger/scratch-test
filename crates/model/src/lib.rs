//! Model to parse the [`*.sb3` file format](https://en.scratch-wiki.info/wiki/Scratch_File_Format)
//! of the [Scratch](https://scratch.mit.edu/) block-oriented programming language
//!
//! (I want to note that I was unable to find the above link to the format specification
//! when I was developing this so I reverse-engineered the format from example files.
//! Luckily, it looks like I correctly understood the meaning of the components.)
//!
//!
//! The program components extracted from a `*.sb3` file are represented by the
//! [`ProjectDoc`] type which offers different methods for parsing a file or
//! a sequence of bytes representing the file's content.
//!
//! ```
//! let doc = ProjectDoc::from_sb3_file("/path/to/file.sb3");
//! println!("{doc:#?}");
//! ```
//!
//! <div class="warning">
//!
//! This project doesn't aim to support all of scratch blocks and when the parsing functions
//! encounter an unknown block or a block that is known to be unsupported those errors
//! won't stop the parsing and will result in a valid document representation.
//! Those invalid blocks are stored differently and may cause problems if
//! the used virtual machine doesn't know how to handle a suddenly not available block.
//!
//! Use for example [`ProjectDoc::ensure_no_invalid_blocks`] on a parsed object to only
//! allow completly usable blocks in the document.
//!
//! ```
//! let doc = ProjectDoc::from_sb3_file("/path/to/file.sb3");
//! println!("Maybe also contains invalid blocks {doc:#?}");
//!
//! match doc.ensure_no_invalid_blocks() {
//!     Ok(doc) => {
//!         println!("Totally valid: {doc:#?}");
//!     }
//!     Err(doc) => {
//!         println!("There are invalid blocks, be extra careful: {doc:#?}");
//!     }
//! }
//! ```
//!
//! </div>
//!
//! # Steps
//!
//! ## JSON extraction
//!
//! A [Scratch file](https://en.scratch-wiki.info/wiki/Scratch_File_Format#Project_Files)
//! is a ZIP file with images, sounds and a `project.json`

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
